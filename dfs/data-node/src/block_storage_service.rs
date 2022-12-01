use cloud_api::error::DataNodeError;
use tokio::sync::mpsc::Sender;
use uuid::Uuid;

use super::data_node::DataNode;

pub struct BlockStorageServiceImpl {
    inner: DataNode,
}

impl BlockStorageServiceImpl {
    pub fn new(block_storage: DataNode) -> Self {
        Self {
            inner: block_storage,
        }
    }

    pub async fn create_block(&self) -> Result<Uuid, DataNodeError> {
        self.inner.create_block().await
    }

    pub async fn read_block(
        &self,
        block_id: &[u8],
        tx: Sender<Result<Vec<u8>, DataNodeError>>,
    ) -> Result<(), DataNodeError> {
        let uuid = Uuid::from_slice(block_id)
            .map_err(|_| DataNodeError::WrongUuid(format!("{:?}", block_id)))?;

        let (path, file_size) = self.inner.get_block_info(uuid).await?;
        let buffer_size = self.inner.get_data_node_info().read_buffer;
        let chunk_count = file_size / buffer_size;
        let last_chunk = file_size - chunk_count * buffer_size;
        for i in 0..(chunk_count + 1) {
            let bytes = if i == chunk_count {
                if last_chunk == 0 {
                    break;
                }

                (i * buffer_size)..(i * buffer_size + last_chunk)
            } else {
                (i * buffer_size)..((i + 1) * buffer_size)
            };

            let read = self.inner.read_block(&path, bytes).await;

            match tx.send(read).await {
                Ok(_) => {
                    tracing::debug!("Sent chunk number {} of {}", i, uuid);
                }
                Err(_) => {
                    tracing::error!("Read stream for {} was dropped", uuid);
                    break;
                }
            }
        }

        Ok(())
    }

    pub async fn update_block(&self, block_id: &[u8], data: &[u8]) -> Result<(), DataNodeError> {
        let uuid = Uuid::from_slice(block_id)
            .map_err(|_| DataNodeError::WrongUuid(format!("{:?}", block_id)))?;
        self.inner.update_block(uuid, data).await
    }

    pub async fn delete_block(&self, block_id: &[u8]) -> Result<(), DataNodeError> {
        let uuid = Uuid::from_slice(block_id)
            .map_err(|_| DataNodeError::WrongUuid(format!("{:?}", block_id)))?;
        self.inner.delete_block(uuid).await
    }
}
