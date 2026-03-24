mod utils;
use tower_lsp_server::LanguageServer;
use tower_lsp_server::ls_types::*;
use utils::*;

#[tokio::test]
async fn test_document_open_and_change() {
    let harness = TestHarness::new().await;

    // Create some files for completion
    harness.create_file("src/lib.rs");
    harness.create_file("src/main.rs");

    // 1. Open a new document
    let content = "let x = \"./\"";
    let uri = harness.open_doc("test.rs", content).await;

    // Verify initial completion works
    harness.assert_completion_contains(&uri, 0, 11, "src").await;

    // 2. Change document content (incremental change)
    // Change "let x = \"./\"" to "let x = \"./sr\""
    harness
        .get_server()
        .did_change(DidChangeTextDocumentParams {
            text_document: VersionedTextDocumentIdentifier {
                uri: uri.clone(),
                version: 2,
            },
            content_changes: vec![TextDocumentContentChangeEvent {
                range: Some(Range {
                    start: Position {
                        line: 0,
                        character: 11,
                    },
                    end: Position {
                        line: 0,
                        character: 11,
                    },
                }),
                range_length: None,
                text: "sr".to_string(),
            }],
        })
        .await;

    // Verify completion after change
    harness.assert_completion_contains(&uri, 0, 13, "src").await;
}

#[tokio::test]
async fn test_document_full_sync() {
    let harness = TestHarness::new().await;
    harness.create_file("data/test.txt");

    let uri = harness.open_doc("main.rs", "").await;

    // Full document sync (range is None)
    harness
        .get_server()
        .did_change(DidChangeTextDocumentParams {
            text_document: VersionedTextDocumentIdentifier {
                uri: uri.clone(),
                version: 2,
            },
            content_changes: vec![TextDocumentContentChangeEvent {
                range: None,
                range_length: None,
                text: "import \"./da\"".to_string(),
            }],
        })
        .await;

    harness
        .assert_completion_contains(&uri, 0, 12, "data")
        .await;
}

#[tokio::test]
async fn test_document_close() {
    let harness = TestHarness::new().await;
    let uri = harness.open_doc("test.rs", "test").await;

    // Close document
    harness
        .get_server()
        .did_close(DidCloseTextDocumentParams {
            text_document: TextDocumentIdentifier { uri: uri.clone() },
        })
        .await;

    // Completion should fail or return error because document is removed
    let params = CompletionParams {
        text_document_position: TextDocumentPositionParams {
            text_document: TextDocumentIdentifier { uri: uri.clone() },
            position: Position {
                line: 0,
                character: 0,
            },
        },
        work_done_progress_params: Default::default(),
        partial_result_params: Default::default(),
        context: None,
    };

    let result = harness.get_server().completion(params).await;
    assert!(
        result.is_err(),
        "Completion should fail for closed document"
    );
}
