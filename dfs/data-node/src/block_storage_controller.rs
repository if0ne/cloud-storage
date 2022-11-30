pub mod proto_data_node {
    tonic::include_proto!("data_node");
}

use super::block_storage_service::BlockStorageServiceImpl;
use super::data_node::DataNode;

use crate::data_node_info::DataNodeInfo;
use proto_data_node::{
    block_storage_service_server::{BlockStorageService, BlockStorageServiceServer},
    CreateBlockRequest, CreateBlockResponse, DeleteBlockRequest, DeleteBlockResponse,
    ReadBlockRequest, ReadBlockResponse, UpdateBlockRequest, UpdateBlockResponse,
};
use tonic::{Request, Response, Status};

pub struct BlockStorageController {
    block_storage: BlockStorageServiceImpl,
}

impl BlockStorageController {
    pub async fn get_service(data_node_info: DataNodeInfo) -> BlockStorageServiceServer<Self> {
        BlockStorageServiceServer::new(Self {
            block_storage: BlockStorageServiceImpl::new(
                DataNode::new(data_node_info).await.unwrap(),
            ),
        })
    }
}

#[tonic::async_trait]
impl BlockStorageService for BlockStorageController {
    async fn create_block(
        &self,
        _: Request<CreateBlockRequest>,
    ) -> Result<Response<CreateBlockResponse>, Status> {
        let uuid = self.block_storage.create_block().await?;
        Ok(Response::new(CreateBlockResponse {
            block_id: uuid.as_bytes().to_vec(),
        }))
    }

    async fn read_block(
        &self,
        request: Request<ReadBlockRequest>,
    ) -> Result<Response<ReadBlockResponse>, Status> {
        let inner = request.into_inner();
        let read = self.block_storage.read_block(&inner.block_id).await?;
        Ok(Response::new(ReadBlockResponse { data: read }))
    }

    async fn update_block(
        &self,
        request: Request<UpdateBlockRequest>,
    ) -> Result<Response<UpdateBlockResponse>, Status> {
        let inner = request.into_inner();
        self.block_storage
            .update_block(&inner.block_id, &inner.data)
            .await?;
        Ok(Response::new(UpdateBlockResponse {}))
    }

    async fn delete_block(
        &self,
        request: Request<DeleteBlockRequest>,
    ) -> Result<Response<DeleteBlockResponse>, Status> {
        let inner = request.into_inner();
        self.block_storage.delete_block(&inner.block_id).await?;
        Ok(Response::new(DeleteBlockResponse {}))
    }
}
