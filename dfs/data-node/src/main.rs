use std::net::SocketAddr;
use tonic::transport::Server;

use data_node::block_storage_controller::BlockStorageController;
use data_node::config::Config;
use data_node::data_node_info::DataNodeInfo;
use data_node::registry_client::RegistryClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    sysinfo::set_open_files_limit(0);
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let config = Config::try_from_file("DataNodeTest.toml").await;
    let registry_client = RegistryClient::new(config.get_main_server_addr()).await;
    match registry_client {
        Ok(mut client) => {
            let response = client.send_registry(&config).await;
            if let Err(response) = response {
                tracing::error!("{}", response.to_string());
            }
        }
        Err(err) => {
            tracing::error!(
                "{}. Please start up the main server and restart data node.",
                err.to_string()
            )
        }
    };

    let data_node_info = DataNodeInfo::new(config).await;
    let addr = format!("{}:{}", data_node_info.self_address, data_node_info.port)
        .parse::<SocketAddr>()
        .expect("Unable to parse socket address");

    let (_, health_service) = tonic_health::server::health_reporter();
    let block_storage_service = BlockStorageController::get_service(data_node_info).await;

    tracing::info!("Starting server on {}:{}", addr.ip(), addr.port());
    Server::builder()
        .accept_http1(true)
        .add_service(health_service)
        .add_service(block_storage_service)
        .serve(addr)
        .await?;

    Ok(())
}
