use lsp_types::{
    ClientCapabilities, ClientInfo, CompletionClientCapabilities, CompletionItemCapability,
    GeneralClientCapabilities, InitializeParams, TextDocumentClientCapabilities,
    TextDocumentSyncClientCapabilities, TraceValue, WindowClientCapabilities,
    WorkspaceClientCapabilities,
    request::{Initialize, Request},
};

use crate::lsp_server::LspServer;

impl LspServer {
    // pub fn initialize(&mut self) -> anyhow::Result<()> {

    //     let params = InitializeParams {
    //         process_id: Some(self.next_id()),
    //         client_info: Some(ClientInfo {
    //             name: "derive".into(),
    //             version: None,
    //         }),
    //         capabilities: Some(ClientCapabilities {
    //             text_document: Some(TextDocumentClientCapabilities {
    //                 synchronization: Some(TextDocumentSyncClientCapabilities {
    //                     did_save: Some(true),
    //                     dynamic_registration: Some(true),
    //                     ..Default::default()
    //                 }),
    //                 completion: Some(CompletionClientCapabilities {
    //                     completion_item: Some( CompletionItemCapability {
    //                         snippet_support: Some(true),
    //                         ..Default::default()
    //                     }),
    //                     ..Default::default()
    //                 }),
    //                 ..Default::default()
    //             }),
    //             workspace: Some(WorkspaceClientCapabilities {
    //                 workspace_folders: Some(true),
    //                 ..Default::default()
    //             }),
    //             ..Default::default()
    //         }),
    //         trace: Some(TraceValue::Off),
    //         ..Default::default()
    //     };
    //     Ok(())
    // }
}
