use crate::block_storage::BlockStorage;
use uuid::Uuid;
use cloud_api::error::BlockStorageError;

pub struct BlockStorageDecorator {
    inner: BlockStorage,
}

impl BlockStorageDecorator {
    pub fn new(block_storage: BlockStorage) -> Self {
        Self {
            inner: block_storage,
        }
    }

    pub async fn create_block(&self, data: &[u8]) -> Result<Uuid, BlockStorageError> {
        self.inner.create_block(data).await
    }

    pub async fn read_block(&self, block_id: &[u8]) -> Result<Vec<u8>, BlockStorageError> {
        let uuid = Uuid::from_slice(block_id).unwrap();
        self.inner.read_block(uuid).await
    }

    pub async fn update_block(&self, block_id: &[u8], data: &[u8]) -> Result<(), BlockStorageError> {
        let uuid = Uuid::from_slice(block_id).unwrap();
        self.inner.update_block(uuid, data).await
    }

    pub async fn delete_block(&self, block_id: &[u8]) -> Result<(), BlockStorageError> {
        let uuid = Uuid::from_slice(block_id).unwrap();
        self.inner.delete_block(uuid).await
    }
}
