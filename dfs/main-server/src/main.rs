mod data_node_client;
mod namespace;
mod namespace_controller;
mod namespace_service;

use std::net::SocketAddr;
use tonic::transport::Server;

use crate::namespace::Namespace;
use crate::namespace_controller::NamespaceController;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));

    Server::builder()
        .accept_http1(true)
        .add_service(NamespaceController::get_service().await)
        .serve(addr)
        .await?;

    Ok(())
}
