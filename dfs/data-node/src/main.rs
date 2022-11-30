use std::net::SocketAddr;
use tonic::transport::Server;

use data_node::block_storage_controller::BlockStorageController;
use data_node::config::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::try_from_file("DataNodeTest.toml").await;
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port()));

    let (_, health_service) = tonic_health::server::health_reporter();
    let block_storage_service = BlockStorageController::get_service(config.port()).await;

    Server::builder()
        .accept_http1(true)
        .add_service(health_service)
        .add_service(block_storage_service)
        .serve(addr)
        .await?;

    Ok(())
}
