use std::{
    collections::HashMap,
    io::{BufReader, BufWriter},
    process::{ChildStdin, Command, Stdio},
    thread::JoinHandle,
};

use anyhow::{Context, anyhow};
use kanal::Sender;

mod commands;
mod initialize;
mod threads;
mod translation;

use crate::{
    event::Event,
    lsp_server::threads::{editor_req_thread, err_thread, io_reader_thread},
};

pub enum EditorRequest {
    Init,
}

enum IOReadOrEditorReq {
    EditorReq(EditorRequest),
    IORead(String),
}

pub fn spawn_server(
    command: String,
    app_sender: kanal::Sender<Event>,
) -> anyhow::Result<kanal::Sender<EditorRequest>> {
    let (mut lsp_server, lsp_sender) = LspServer::new(command, app_sender)?;

    fn inner(lsp_server: &mut LspServer) -> anyhow::Result<()> {
        loop {
            match lsp_server.lsp_recv.recv()? {
                IOReadOrEditorReq::EditorReq(editor_req) => {
                    lsp_server.process_editor_req(editor_req)?
                }
                IOReadOrEditorReq::IORead(lsp_read) => {
                    lsp_server.app_sender.send(Event::Lsp(lsp_read))?
                }
            };
        }
    }

    std::thread::spawn(move || match inner(&mut lsp_server) {
        Ok(_) => (),
        Err(err) => {
            // Can be ignored when already erroring on sending errors, sending the error is useless.
            let _ = lsp_server
                .app_sender
                .send(Event::Error(anyhow!("LSP Error: {}", err)));
        }
    });
    Ok(lsp_sender)
}

struct LspServer {
    writer: BufWriter<ChildStdin>,
    app_sender: kanal::Sender<Event>,
    lsp_recv: kanal::Receiver<IOReadOrEditorReq>,
    pending: HashMap<u64, &'static str>,
    next_id: u32,
    handles: Vec<JoinHandle<()>>,
    ready: bool,
}

impl LspServer {
    fn new(
        command: String,
        app_sender: kanal::Sender<Event>,
    ) -> anyhow::Result<(Self, Sender<EditorRequest>)> {
        let lsp = Command::new(command)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let writer = lsp.stdin.map(BufWriter::new).context("No Writer")?;
        let reader = lsp.stdout.map(BufReader::new).context("No Reader")?;
        let err_reader = lsp.stderr.map(BufReader::new).context("No Stderr")?;

        let (lsp_sender, lsp_recv) = kanal::unbounded::<IOReadOrEditorReq>();
        let (editor_req_sender, editor_req_recv) = kanal::unbounded::<EditorRequest>();

        let handles = vec![
            err_thread(err_reader, app_sender.clone()),
            io_reader_thread(reader, lsp_sender.clone(), app_sender.clone()),
            editor_req_thread(editor_req_recv, lsp_sender.clone(), app_sender.clone()),
        ];

        Ok((
            Self {
                app_sender,
                lsp_recv,
                writer,
                pending: HashMap::new(),
                next_id: 0,
                ready: false,
                handles,
            },
            editor_req_sender,
        ))
    }
    pub fn next_id(&mut self) -> u32 {
        self.next_id += 1;
        self.next_id - 1
    }

    fn process_editor_req(&mut self, editor_req: EditorRequest) -> anyhow::Result<()> {
        match editor_req {
            EditorRequest::Init => self.initialize(),
        }
    }
}

impl Drop for LspServer {
    fn drop(&mut self) {
        // drop the handles (switch to tokyo handles)
    }
}
