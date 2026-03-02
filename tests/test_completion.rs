mod utils;
use utils::*;

#[tokio::test]
async fn test_simple_relative_completion() {
    let harness = TestHarness::new().await;

    harness.create_file("data/config.json");
    harness.create_file("src/main.rs");

    // edit: src/main.rs
    let content = "let f = \"./da";
    let uri = harness.open_doc("src/main.rs", content).await;

    harness
        .assert_completion_contains(&uri, 0, 12, "data")
        .await;
}

#[tokio::test]
async fn test_sibling_file_completion() {
    let harness = TestHarness::new().await;

    harness.create_file("images/logo.png");
    harness.create_file("README.md");

    let content = "Check ./RE";
    let uri = harness.open_doc("images/info.txt", content).await;

    harness
        .assert_completion_contains(&uri, 0, 11, "README.md")
        .await;
}
