use crate::data_node::DataNode;

use cloud_api::error::DataNodeError;
use uuid::Uuid;

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

    pub async fn read_block(&self, block_id: &[u8]) -> Result<Vec<u8>, DataNodeError> {
        let uuid = Uuid::from_slice(block_id)
            .map_err(|_| DataNodeError::WrongUuid(format!("{:?}", block_id)))?;
        self.inner.read_block(uuid).await
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
