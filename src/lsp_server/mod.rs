use std::{
    collections::HashMap,
    io::{BufReader, BufWriter, Read, Write},
    process::{ChildStdin, Command, Stdio},
};

use anyhow::{Context, anyhow};
use kanal::Sender;

mod commands;
mod initialize;

use crate::event::Event;

pub enum EditorRequest {
    Init,
}

pub fn spawn_server(
    command: String,
    app_sender: kanal::Sender<Event>,
) -> anyhow::Result<kanal::Sender<EditorRequest>> {
    let (mut lsp_server, lsp_sender) = LspServer::new(command, app_sender)?;

    fn inner(lsp_server: &mut LspServer) -> anyhow::Result<()> {
        loop {
            let editor_req = lsp_server.lsp_recv.recv()?;
            lsp_server.process_editor_req(editor_req)?;
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
    lsp_sender.send(EditorRequest::Init)?;
    Ok(lsp_sender)
}

struct LspServer {
    writer: BufWriter<ChildStdin>,
    app_sender: kanal::Sender<Event>,
    lsp_recv: kanal::Receiver<EditorRequest>,
    pending: HashMap<u64, &'static str>,
    next_id: u32,
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
        let mut err_reader = lsp.stderr.map(BufReader::new).context("No Stderr")?;

        let err_app_sender = app_sender.clone();
        std::thread::spawn(move || {
            loop {
                let mut buf = String::new();
                err_reader.read_to_string(&mut buf).unwrap();
                err_app_sender
                    .send(Event::Error(anyhow!("Stdio error with LSP: {}", buf)))
                    .unwrap();
            }
        });

        let (lsp_sender, lsp_recv) = kanal::unbounded::<EditorRequest>();
        Ok((
            Self {
                app_sender,
                lsp_recv,
                writer,
                pending: HashMap::new(),
                next_id: 0,
                ready: false,
            },
            lsp_sender,
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
