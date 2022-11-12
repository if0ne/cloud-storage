use proto::{
    CreateBlockRequest, CreateBlockResponse, ReadBlockRequest, ReadBlockResponse, UpdateBlockRequest, UpdateBlockResponse,
    DeleteBlockRequest, DeleteBlockResponse, block_storage_service_server::{BlockStorageService, BlockStorageServiceServer}
};
use tonic::{Request, Response, Status};
use crate::block_storage::StorageTag;
use crate::BlockStorage;

pub mod proto {
    tonic::include_proto!("data_node");
}

pub struct BlockStorageServiceImpl {
    block_storage: BlockStorage
}

impl BlockStorageServiceImpl {
    pub async fn get_service(tag: StorageTag) -> BlockStorageServiceServer<Self> {
        BlockStorageServiceServer::new(BlockStorageServiceImpl { block_storage: BlockStorage::new(tag).await })
    }
}

#[tonic::async_trait]
impl BlockStorageService for BlockStorageServiceImpl {
    async fn create_block(&self, request: Request<CreateBlockRequest>) -> Result<Response<CreateBlockResponse>, Status> {
        let inner = request.into_inner();
        self.block_storage.create_block(inner.block_id, &inner.data).await;
        Ok(Response::new(CreateBlockResponse {}))
    }

    async fn read_block(&self, request: Request<ReadBlockRequest>) -> Result<Response<ReadBlockResponse>, Status> {
        let inner = request.into_inner();
        let read = self.block_storage.read_block(inner.block_id).await;
        Ok(Response::new(ReadBlockResponse {
            data: read
        }))
    }

    async fn update_block(&self, request: Request<UpdateBlockRequest>) -> Result<Response<UpdateBlockResponse>, Status> {
        let inner = request.into_inner();
        self.block_storage.update_block(inner.block_id, &inner.data).await;
        Ok(Response::new(UpdateBlockResponse {}))
    }

    async fn delete_block(&self, request: Request<DeleteBlockRequest>) -> Result<Response<DeleteBlockResponse>, Status> {
        let inner = request.into_inner();
        self.block_storage.delete_block(inner.block_id).await;
        Ok(Response::new(DeleteBlockResponse {}))
    }
}