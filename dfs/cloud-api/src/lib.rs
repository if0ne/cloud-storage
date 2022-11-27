pub mod client;
pub mod error;

pub mod proto_data_node {
    tonic::include_proto!("data_node");
}

pub mod proto_main_server {
    tonic::include_proto!("main_server");
}

use async_trait::async_trait;
use std::ops::Range;
use std::path::Path;
use tokio::io::{AsyncReadExt, AsyncWrite};
use tonic::transport::Endpoint;

use proto_main_server::namespace_service_client::NamespaceServiceClient;
use crate::client::CloudClient;
use crate::proto_main_server::{CreateFileRequest, NewBlockRequest};

pub struct Client {
    main_server_client: NamespaceServiceClient<tonic::transport::Channel>,
}

impl Client {
    pub async fn new(addr: &'static str) -> Self {
        let endpoint = Endpoint::try_from(addr).unwrap();
        let channel = endpoint.connect().await.unwrap();
        
        Self {
            main_server_client: NamespaceServiceClient::new(channel)
        }
    }
}

#[async_trait]
impl CloudClient for Client {
    async fn make_dir(&mut self, path: impl AsRef<Path> + Send) {
        todo!()
    }

    async fn create_small_file(&mut self, path: impl AsRef<Path> + Send + Sync) {
        let result = self.main_server_client.create_small_file(CreateFileRequest {
            path: path.as_ref().to_str().unwrap().to_string()
        }).await.unwrap().into_inner();
    }

    async fn create_small_file_from_stream(&mut self, path: impl AsRef<Path> + Send) {
        todo!()
    }

    async fn create_large_file(&mut self, path: impl AsRef<Path> + Send) {
        todo!()
    }

    async fn create_large_file_from_stream(&mut self, path: impl AsRef<Path> + Send) {
        todo!()
    }

    async fn read_small_file(&mut self, path: impl AsRef<Path> + Send) {
        todo!()
    }

    async fn read_large_file(&mut self, path: impl AsRef<Path> + Send, blocks: Range<usize>) {
        todo!()
    }

    async fn commit_to_small_file(&mut self, path: impl AsRef<Path> + Send, data: impl AsRef<[u8]> + Send) {
        todo!()
    }

    async fn commit_to_small_file_from_stream(&mut self, path: impl AsRef<Path> + Send, stream: impl AsyncReadExt + AsyncWrite + Send) {
        todo!()
    }
}