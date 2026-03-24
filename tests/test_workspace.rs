mod utils;
use tower_lsp_server::LanguageServer;
use tower_lsp_server::ls_types::*;
use utils::*;

#[tokio::test]
async fn test_workspace_folder_added() {
    let harness = TestHarness::new().await;

    // Create an external folder for workspace
    let temp_folder = tempfile::tempdir().unwrap();
    let folder_path = temp_folder.path().to_path_buf();
    let folder_uri = Uri::from_file_path(&folder_path).unwrap();

    // Create a file in this new folder
    let data_file = folder_path.join("external_data/config.json");
    std::fs::create_dir_all(data_file.parent().unwrap()).unwrap();
    std::fs::File::create(data_file).unwrap();

    // 1. Add workspace folder to the server
    harness
        .get_server()
        .did_change_workspace_folders(DidChangeWorkspaceFoldersParams {
            event: WorkspaceFoldersChangeEvent {
                added: vec![WorkspaceFolder {
                    uri: folder_uri.clone(),
                    name: "external".to_string(),
                }],
                removed: vec![],
            },
        })
        .await;

    // 2. Try completion in the current workspace referencing the new workspace folder
    let content = "let config = \"./external_da";
    let uri = harness.open_doc("main.rs", content).await;

    // The current completion implementation should ideally support resolving from any workspace root
    harness
        .assert_completion_contains(&uri, 0, content.len() as u32, "external_data")
        .await;
}

#[tokio::test]
async fn test_workspace_folder_removed() {
    let harness = TestHarness::new().await;

    // Add external folder
    let temp_folder = tempfile::tempdir().unwrap();
    let folder_path = temp_folder.path().to_path_buf();
    let folder_uri = Uri::from_file_path(&folder_path).unwrap();
    let data_file = folder_path.join("removed_data/config.json");
    std::fs::create_dir_all(data_file.parent().unwrap()).unwrap();
    std::fs::File::create(data_file).unwrap();

    harness
        .get_server()
        .did_change_workspace_folders(DidChangeWorkspaceFoldersParams {
            event: WorkspaceFoldersChangeEvent {
                added: vec![WorkspaceFolder {
                    uri: folder_uri.clone(),
                    name: "external".to_string(),
                }],
                removed: vec![],
            },
        })
        .await;

    // Remove folder
    harness
        .get_server()
        .did_change_workspace_folders(DidChangeWorkspaceFoldersParams {
            event: WorkspaceFoldersChangeEvent {
                added: vec![],
                removed: vec![WorkspaceFolder {
                    uri: folder_uri.clone(),
                    name: "external".to_string(),
                }],
            },
        })
        .await;

    let content = "\"./removed_da";
    let uri = harness.open_doc("main.rs", content).await;

    // Completion should NOT find it
    let result = harness
        .get_server()
        .completion(CompletionParams {
            text_document_position: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier { uri: uri.clone() },
                position: Position {
                    line: 0,
                    character: content.len() as u32,
                },
            },
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
            context: None,
        })
        .await
        .unwrap();

    let items = match result {
        Some(CompletionResponse::Array(items)) => items,
        Some(CompletionResponse::List(list)) => list.items,
        None => vec![],
    };

    let found = items.iter().any(|item| item.label == "removed_data");
    assert!(
        !found,
        "Completion should not return results for removed workspace root"
    );
}
