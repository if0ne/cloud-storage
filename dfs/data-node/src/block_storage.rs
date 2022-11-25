use std::io::SeekFrom;
use std::path::Path;

use cloud_api::error::BlockStorageError;
use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt, BufReader, BufWriter};
use uuid::Uuid;

pub type StorageTag = u16;

pub struct BlockStorage {
    tag: StorageTag,
}

impl BlockStorage {
    pub async fn new(tag: StorageTag) -> std::io::Result<Self> {
        let inner_path = format!("{}", tag);
        let path = Path::new(&inner_path);

        if !path.exists() {
            tokio::fs::create_dir(inner_path).await?;
        }

        Ok(Self { tag })
    }

    pub async fn create_block(&self, data: &[u8]) -> Result<Uuid, BlockStorageError> {
        let uuid = Uuid::new_v4();
        let file = OpenOptions::new()
            .write(true)
            .read(false)
            .create(true)
            .open(format!("{}/{}", self.tag, uuid.as_u128()))
            .await
            .map_err(|err| BlockStorageError::CreateBlockError(err.to_string()))?;
        self.write_block(file, data).await?;

        Ok(uuid)
    }

    pub async fn read_block(&self, block_id: Uuid) -> Result<Vec<u8>, BlockStorageError> {
        let file = OpenOptions::new()
            .write(false)
            .read(true)
            .open(format!("{}/{}", self.tag, block_id.as_u128()))
            .await
            .map_err(|err| BlockStorageError::BlockNotFound(err.to_string()))?;
        let buffer_len = file
            .metadata()
            .await
            .map_err(|err| BlockStorageError::ReadBlockError(err.to_string()))?
            .len() as usize;
        let mut reader = BufReader::new(file);
        let mut buffer = vec![0; buffer_len];
        reader
            .read_exact(&mut buffer)
            .await
            .map_err(|err| BlockStorageError::ReadBlockError(err.to_string()))?;

        Ok(buffer)
    }

    pub async fn update_block(&self, block_id: Uuid, data: &[u8]) -> Result<(), BlockStorageError> {
        let file = OpenOptions::new()
            .write(true)
            .read(false)
            .open(format!("{}/{}", self.tag, block_id.as_u128()))
            .await
            .map_err(|err| BlockStorageError::DeleteBlockError(err.to_string()))?;
        self.write_block(file, data).await
    }

    pub async fn delete_block(&self, block_id: Uuid) -> Result<(), BlockStorageError> {
        tokio::fs::remove_file(format!("{}/{}", self.tag, block_id.as_u128()))
            .await
            .map_err(|err| BlockStorageError::DeleteBlockError(err.to_string()))
    }

    #[inline]
    async fn write_block(&self, file: File, data: &[u8]) -> Result<(), BlockStorageError> {
        let mut writer = BufWriter::new(file);

        writer
            .seek(SeekFrom::Start(0))
            .await
            .map_err(|err| BlockStorageError::UpdateBlockError(err.to_string()))?;
        writer
            .write_all(data)
            .await
            .map_err(|err| BlockStorageError::UpdateBlockError(err.to_string()))?;
        writer
            .flush()
            .await
            .map_err(|err| BlockStorageError::UpdateBlockError(err.to_string()))?;

        Ok(())
    }
}

#[cfg(any(test, bench))]
impl Drop for BlockStorage {
    fn drop(&mut self) {
        std::fs::remove_dir_all(format!("{}", self.tag)).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_block_crud() {
        let first_record = b"Hello, World";
        let second_record = b"Hello, Pavel";

        let block_storage = BlockStorage::new(40000).await.unwrap();
        let uuid = block_storage.create_block(first_record).await;
        assert_eq!(
            first_record.as_slice(),
            block_storage.read_block(uuid).await.as_slice()
        );
        block_storage.update_block(uuid, second_record).await;
        assert_eq!(
            second_record.as_slice(),
            block_storage.read_block(uuid).await.as_slice()
        );
        block_storage.delete_block(uuid).await;
    }
}
