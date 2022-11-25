use uuid::Uuid;
use crate::block_storage::BlockStorage;

pub struct BlockStorageDecorator {
    inner: BlockStorage
}

impl BlockStorageDecorator {
    pub fn new(block_storage: BlockStorage) -> Self {
        Self {
            inner: block_storage
        }
    }

    pub async fn create_block(&self, data: &[u8]) -> Uuid {
        self.inner.create_block(data).await
    }

    pub async fn read_block(&self, block_id: &[u8]) -> Vec<u8> {
        let uuid = Uuid::from_slice(block_id).unwrap();
        self.inner.read_block(uuid).await
    }

    pub async fn update_block(&self, block_id: &[u8], data: &[u8]) {
        let uuid = Uuid::from_slice(block_id).unwrap();
        self.inner.update_block(uuid, data).await
    }

    pub async fn delete_block(&self, block_id: &[u8]) {
        let uuid = Uuid::from_slice(block_id).unwrap();
        self.inner.delete_block(uuid).await
    }
}