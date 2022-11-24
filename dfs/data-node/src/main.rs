use clap::Parser;
use std::net::SocketAddr;
use tonic::transport::Server;

use data_node::block_storage_service::BlockStorageServiceImpl;

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
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config: Config = Config::parse();
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));

    Server::builder()
        .accept_http1(true)
        .add_service(BlockStorageServiceImpl::get_service(config.port).await)
        .serve(addr)
        .await?;

    Ok(())
}
