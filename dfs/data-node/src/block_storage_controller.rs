pub mod proto_data_node {
    tonic::include_proto!("data_node");
}

use futures::StreamExt;
use proto_data_node::{
    block_storage_service_server::{BlockStorageService, BlockStorageServiceServer},
    CreateBlockRequest, CreateBlockResponse, DeleteBlockRequest, DeleteBlockResponse,
    ReadBlockRequest, ReadBlockResponse, UpdateBlockRequest, UpdateBlockResponse,
};
use std::sync::Arc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};
use uuid::Uuid;

use super::block_storage_service::BlockStorageServiceImpl;
use super::data_node::DataNode;
use super::data_node_info::DataNodeInfo;

pub struct BlockStorageController {
    block_storage: Arc<BlockStorageServiceImpl>,
}

impl BlockStorageController {
    pub async fn get_service(data_node_info: DataNodeInfo) -> BlockStorageServiceServer<Self> {
        BlockStorageServiceServer::new(Self {
            block_storage: Arc::new(BlockStorageServiceImpl::new(
                DataNode::new(data_node_info).await.unwrap(),
            )),
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

    type ReadBlockStream = ReceiverStream<Result<ReadBlockResponse, Status>>;

    async fn read_block(
        &self,
        request: Request<ReadBlockRequest>,
    ) -> Result<Response<Self::ReadBlockStream>, Status> {
        let inner = request.into_inner();

        let (controller_tx, controller_rx) = tokio::sync::mpsc::channel(128);
        let (service_tx, service_rx) = tokio::sync::mpsc::channel(128);
        let block_storage = self.block_storage.clone();

        tokio::spawn(async move {
            let response = block_storage.read_block(&inner.block_id, service_tx).await;
            if let Err(err) = response {
                let response = controller_tx.send(Err(err.into())).await;
                if let Err(err) = response {
                    tracing::debug!("Send error while reading: {}", err);
                    return;
                }
            }
            let mut stream = ReceiverStream::new(service_rx);
            while let Some(response) = stream.next().await {
                let response = controller_tx
                    .send(
                        response
                            .map(|buffer| ReadBlockResponse { data: buffer })
                            .map_err(|err| err.into()),
                    )
                    .await;

                match response {
                    Ok(_) => {
                        tracing::debug!(
                            "Sent data chunk for {}",
                            Uuid::from_slice(&inner.block_id).unwrap()
                        );
                    }
                    Err(_) => {
                        tracing::error!(
                            "Stream for {} was dropped",
                            Uuid::from_slice(&inner.block_id).unwrap()
                        );
                        break;
                    }
                }
            }
        });

        Ok(Response::new(ReceiverStream::new(controller_rx)))
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
