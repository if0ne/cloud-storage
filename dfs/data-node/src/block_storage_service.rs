use crate::block_storage::BlockStorage;

use cloud_api::error::BlockStorageError;
use uuid::Uuid;

pub struct BlockStorageServiceImpl {
    inner: BlockStorage,
}

impl BlockStorageServiceImpl {
    pub fn new(block_storage: BlockStorage) -> Self {
        Self {
            inner: block_storage,
        }
    }

    pub async fn create_block(&self) -> Result<Uuid, BlockStorageError> {
        self.inner.create_block().await
    }

    pub async fn read_block(&self, block_id: &[u8]) -> Result<Vec<u8>, BlockStorageError> {
        let uuid = Uuid::from_slice(block_id)
            .map_err(|_| BlockStorageError::WrongUuid(format!("{:?}", block_id)))?;
        self.inner.read_block(uuid).await
    }

    pub async fn update_block(
        &self,
        block_id: &[u8],
        data: &[u8],
    ) -> Result<(), BlockStorageError> {
        let uuid = Uuid::from_slice(block_id)
            .map_err(|_| BlockStorageError::WrongUuid(format!("{:?}", block_id)))?;
        self.inner.update_block(uuid, data).await
    }

    pub async fn delete_block(&self, block_id: &[u8]) -> Result<(), BlockStorageError> {
        let uuid = Uuid::from_slice(block_id)
            .map_err(|_| BlockStorageError::WrongUuid(format!("{:?}", block_id)))?;
        self.inner.delete_block(uuid).await
    }
}
