use crate::block_storage::BlockStorage;
use crate::block_storage::StorageTag;

use proto::{
    block_storage_service_server::{BlockStorageService, BlockStorageServiceServer},
    CreateBlockRequest, CreateBlockResponse, DeleteBlockRequest, DeleteBlockResponse,
    ReadBlockRequest, ReadBlockResponse, UpdateBlockRequest, UpdateBlockResponse,
};
use tonic::{Request, Response, Status};
use uuid::Uuid;

pub mod proto {
    tonic::include_proto!("data_node");
}

pub struct BlockStorageServiceImpl {
    block_storage: BlockStorage,
}

impl BlockStorageServiceImpl {
    pub async fn get_service(tag: StorageTag) -> BlockStorageServiceServer<Self> {
        BlockStorageServiceServer::new(BlockStorageServiceImpl {
            block_storage: BlockStorage::new(tag).await,
        })
    }
}

#[tonic::async_trait]
impl BlockStorageService for BlockStorageServiceImpl {
    async fn create_block(
        &self,
        request: Request<CreateBlockRequest>,
    ) -> Result<Response<CreateBlockResponse>, Status> {
        let inner = request.into_inner();
        let uuid = self.block_storage.create_block(&inner.data).await;
        Ok(Response::new(CreateBlockResponse {
            block_id: uuid.as_bytes().to_vec(),
        }))
    }

    async fn read_block(
        &self,
        request: Request<ReadBlockRequest>,
    ) -> Result<Response<ReadBlockResponse>, Status> {
        let inner = request.into_inner();
        let read = self
            .block_storage
            .read_block(Uuid::from_slice(&inner.block_id).unwrap())
            .await;
        Ok(Response::new(ReadBlockResponse { data: read }))
    }

    async fn update_block(
        &self,
        request: Request<UpdateBlockRequest>,
    ) -> Result<Response<UpdateBlockResponse>, Status> {
        let inner = request.into_inner();
        self.block_storage
            .update_block(Uuid::from_slice(&inner.block_id).unwrap(), &inner.data)
            .await;
        Ok(Response::new(UpdateBlockResponse {}))
    }

    async fn delete_block(
        &self,
        request: Request<DeleteBlockRequest>,
    ) -> Result<Response<DeleteBlockResponse>, Status> {
        let inner = request.into_inner();
        self.block_storage
            .delete_block(Uuid::from_slice(&inner.block_id).unwrap())
            .await;
        Ok(Response::new(DeleteBlockResponse {}))
    }
}
