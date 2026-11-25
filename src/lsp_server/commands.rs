use std::io::Write;

use serde::Serialize;
use serde_json::Value;

use crate::lsp_server::LspServer;

#[derive(Serialize)]
pub enum LspCommand {
    Request {
        jsonrpc: String,
        id: u32,
        method: String,
        params: Value,
    },
    Response,
}

impl LspServer {
    fn send(&mut self, command: LspCommand) -> anyhow::Result<()> {
        let str = serde_json::to_string(&command)?;
        self.writer.write(str.as_bytes())?;
        self.writer.flush()?;
        Ok(())
    }
}
