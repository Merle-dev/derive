use lsp_types::{
    ClientCapabilities, ClientInfo, CompletionClientCapabilities, CompletionItemCapability,
    InitializeParams, TextDocumentClientCapabilities, TextDocumentSyncClientCapabilities,
    TraceValue, WorkspaceClientCapabilities,
};
use serde_json::Value;

use crate::lsp_server::{LspServer, commands::LspCommand};

impl LspServer {
    pub fn initialize(&mut self) -> anyhow::Result<()> {
        let command = LspCommand::Request {
            jsonrpc: "2.0".to_string(),
            id: self.next_id(),
            method: "initialize".to_string(),
            params: self.initialize_params()?,
        };

        self.app_sender
            .send(crate::event::Event::Info(serde_json::to_string(&command)?))
            .unwrap();

        self.send(command)?;
        Ok(())
    }
    fn initialize_params(&mut self) -> anyhow::Result<Value> {
        let params = InitializeParams {
            process_id: Some(std::process::id()),
            client_info: Some(ClientInfo {
                name: "derive".into(),
                version: None,
            }),
            capabilities: ClientCapabilities {
                text_document: Some(TextDocumentClientCapabilities {
                    synchronization: Some(TextDocumentSyncClientCapabilities {
                        did_save: Some(true),
                        dynamic_registration: Some(true),
                        ..Default::default()
                    }),
                    completion: Some(CompletionClientCapabilities {
                        completion_item: Some(CompletionItemCapability {
                            snippet_support: Some(true),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                workspace: Some(WorkspaceClientCapabilities {
                    workspace_folders: Some(true),
                    ..Default::default()
                }),
                ..Default::default()
            },
            trace: Some(TraceValue::Off),
            ..Default::default()
        };
        Ok(serde_json::to_value(params)?)
    }
}
