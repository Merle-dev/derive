use std::{
    io::{BufRead, BufReader},
    process::{ChildStderr, ChildStdout},
};

use anyhow::anyhow;
use kanal::{Receiver, Sender};

use crate::{
    event::Event,
    lsp_server::{EditorRequest, IOReadOrEditorReq, translation::io_read},
};

pub(super) fn err_thread(
    mut err_reader: BufReader<ChildStderr>,
    app_sender: Sender<Event>,
) -> std::thread::JoinHandle<()> {
    std::thread::spawn(move || {
        let mut line = String::new();
        loop {
            line.clear();
            match err_reader.read_line(&mut line) {
                Ok(0) => break, // EOF: LSP closed stderr
                Ok(_) => {
                    let msg = line.trim_end_matches(['\r', '\n']);
                    if !msg.is_empty()
                        && app_sender
                            .send(Event::Error(anyhow!("LSP stderr: {msg}")))
                            .is_err()
                    {
                        break;
                    }
                }
                Err(err) => {
                    let _ = app_sender.send(Event::Error(anyhow!("LSP stderr read error: {err}")));
                    break;
                }
            }
        }
    })
}

pub(crate) fn io_reader_thread(
    mut reader: BufReader<ChildStdout>,
    lsp_sender: Sender<IOReadOrEditorReq>,
    app_sender: Sender<Event>,
) -> std::thread::JoinHandle<()> {
    std::thread::spawn(move || {
        loop {
            match io_read(&mut reader) {
                Ok(Some(payload)) => {
                    dbg!(&payload);
                    if let Err(err) = lsp_sender.send(IOReadOrEditorReq::IORead(payload)) {
                        app_sender.send(Event::Error(err.into())).unwrap();
                        break;
                    }
                }
                Ok(None) => break,
                Err(err) => app_sender.send(Event::Error(err)).unwrap(),
            }
        }
    })
}

pub(crate) fn editor_req_thread(
    editor_req_recv: Receiver<EditorRequest>,
    lsp_sender: Sender<IOReadOrEditorReq>,
    app_sender: Sender<Event>,
) -> std::thread::JoinHandle<()> {
    std::thread::spawn(move || {
        while let Ok(editor_req) = editor_req_recv.recv() {
            if let Err(err) = lsp_sender.send(IOReadOrEditorReq::EditorReq(editor_req)) {
                app_sender.send(Event::Error(err.into())).unwrap();
                break;
            }
        }
    })
}
