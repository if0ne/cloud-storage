use std::net::SocketAddr;
use tonic::transport::Server;

use crate::namespace::Namespace;
use crate::namespace_controller::NamespaceController;

mod namespace;
mod namespace_service;
mod namespace_controller;

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
