use proto::{
    CreateBlockRequest, CreateBlockResponse, ReadBlockRequest, ReadBlockResponse, UpdateBlockRequest, UpdateBlockResponse,
    DeleteBlockRequest, DeleteBlockResponse, block_storage_service_server::{BlockStorageService, BlockStorageServiceServer}
};
use tonic::{Request, Response, Status};
use crate::BlockStorage;

pub mod proto {
    tonic::include_proto!("data_node");
}

pub struct BlockStorageServiceImpl {}

impl BlockStorageServiceImpl {
    pub fn get_service() -> BlockStorageServiceServer<Self> {
        BlockStorageServiceServer::new(BlockStorageServiceImpl {})
    }
}

#[tonic::async_trait]
impl BlockStorageService for BlockStorageServiceImpl {
    async fn create_block(&self, request: Request<CreateBlockRequest>) -> Result<Response<CreateBlockResponse>, Status> {
        let storage = BlockStorage::new(40000).await;
        let inner = request.into_inner();
        dbg!(&inner);
        storage.create_block(inner.block_id, &inner.data).await;
        Ok(Response::new(CreateBlockResponse {}))
    }

    async fn read_block(&self, request: Request<ReadBlockRequest>) -> Result<Response<ReadBlockResponse>, Status> {
        let storage = BlockStorage::new(40000).await;
        let inner = request.into_inner();
        let read = storage.read_block(inner.block_id).await;
        Ok(Response::new(ReadBlockResponse {
            data: read
        }))
    }

    async fn update_block(&self, request: Request<UpdateBlockRequest>) -> Result<Response<UpdateBlockResponse>, Status> {
        let storage = BlockStorage::new(40000).await;
        let inner = request.into_inner();
        storage.update_block(inner.block_id, &inner.data).await;
        Ok(Response::new(UpdateBlockResponse {}))
    }

    async fn delete_block(&self, request: Request<DeleteBlockRequest>) -> Result<Response<DeleteBlockResponse>, Status> {
        let storage = BlockStorage::new(40000).await;
        let inner = request.into_inner();
        storage.delete_block(inner.block_id).await;
        Ok(Response::new(DeleteBlockResponse {}))
    }
}