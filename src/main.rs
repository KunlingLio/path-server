use path_server::PathServer;

#[cfg(feature = "multi-thread")]
#[tokio::main(flavor = "multi_thread")]
async fn main() {
    inner().await;
}

#[cfg(not(feature = "multi-thread"))]
#[tokio::main(flavor = "current_thread")]
async fn main() {
    inner().await;
}

async fn inner() {
    unsafe {
        std::env::set_var("RUST_BACKTRACE", "1");
    }

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = tower_lsp::LspService::new(PathServer::new);
    tower_lsp::Server::new(stdin, stdout, socket)
        .serve(service)
        .await;
}
