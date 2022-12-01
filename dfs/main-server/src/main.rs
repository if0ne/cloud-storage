mod data_node_client;
mod namespace;
mod namespace_controller;
mod namespace_service;
mod registry_controller;

use std::net::{SocketAddr, ToSocketAddrs};
use tonic::transport::Server;

use crate::namespace::Namespace;
use crate::namespace_controller::NamespaceController;
use crate::registry_controller::RegistryController;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let addr = "[::1]:8000".to_socket_addrs().unwrap().next().unwrap();

    Server::builder()
        .add_service(RegistryController::get_service().await)
        .add_service(NamespaceController::get_service().await)
        .serve(addr)
        .await?;

    Ok(())
}
