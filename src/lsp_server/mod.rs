use std::{
    collections::HashMap,
    io::{BufReader, BufWriter, Read, Write},
    process::{ChildStdin, Command, Stdio},
};

use anyhow::Context;

use crate::event::Event;

enum EditorRequest {}

pub fn spawn_server(
    command: String,
    app_sender: kanal::Sender<Event>,
) -> kanal::Sender<EditorRequest> {
    todo!()
}

struct LspServer {
    writer: BufWriter<ChildStdin>,
    app_sender: kanal::Sender<Event>,
    pending: HashMap<u64, &'static str>,
    next_id: u64,
    ready: bool,
}

impl LspServer {
    fn new(command: String, app_sender: kanal::Sender<Event>) -> anyhow::Result<Self> {
        let lsp = Command::new(command)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let writer = lsp.stdin.map(BufWriter::new).context("No Writer")?;
        let reader = lsp.stdout.map(BufReader::new).context("No Reader")?;
        let mut err_reader = lsp.stderr.map(BufReader::new).context("No Stderr")?;

        std::thread::spawn(move || {
            let mut file = std::fs::File::create("lsp_err.log").unwrap();
            loop {
                let mut buf = String::new();
                err_reader.read_to_string(&mut buf).unwrap();
                file.write(buf.as_bytes()).unwrap();
            }
        });

        Ok(Self {
            app_sender,
            writer,
            pending: HashMap::new(),
            next_id: 0,
            ready: false,
        })
    }
}
