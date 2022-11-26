pub mod proto_data_node {
    tonic::include_proto!("data_node");
}

use proto_data_node::{
    block_storage_service_client::BlockStorageServiceClient, CreateBlockRequest,
    CreateBlockResponse,
};
use tonic::transport::Endpoint;
use uuid::Uuid;

pub struct DataNodeClient {
    inner: BlockStorageServiceClient<tonic::transport::Channel>,
}

impl DataNodeClient {
    //TODO: Error Handling
    pub async fn new(endpoint: Endpoint) -> Self {
        let channel = endpoint.connect().await.unwrap();
        let client = BlockStorageServiceClient::new(channel);

        Self { inner: client }
    }

    pub async fn new_block(&mut self) -> Uuid {
        let block_id = self
            .inner
            .create_block(CreateBlockRequest {})
            .await
            .unwrap();
        Uuid::from_slice(&block_id.into_inner().block_id).unwrap(/*Can not panic*/)
    }
}
