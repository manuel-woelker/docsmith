use docsmith_server::server::DocsmithServer;

fn main() {
    tokio::runtime::Runtime::new().unwrap().block_on(async_main());
}

async fn async_main() {
    let result = DocsmithServer::new().run().await;
    if let Err(e) = result {
        eprintln!("Error: {}", e);
    }
}