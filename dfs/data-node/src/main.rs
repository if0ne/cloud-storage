mod block_storage;

use axum::routing::get;
use axum::Router;
use clap::Parser;
use std::net::SocketAddr;

use crate::block_storage::BlockStorage;

#[derive(Parser, Debug)]
struct Config {
    // Address of main server
    #[arg(short, long)]
    main_server_address: String,

    /// Port
    #[arg(short, long, default_value_t = 40000)]
    port: u16,

    /// Volume of disk space to use in KB. If not set service will use all disk space
    #[arg(short, long)]
    disk_usage: Option<u64>,
}

#[tokio::main]
async fn main() {
    let config: Config = Config::parse();
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));

    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    let block_storage = BlockStorage::new(config.port).await;

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal(block_storage))
        .await
        .unwrap()
}

async fn shutdown_signal(block_storage: BlockStorage) {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install Ctrl+C handler");
}
