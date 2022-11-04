use std::io::SeekFrom;
use std::path::Path;

use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt, BufReader, BufWriter};

pub type StorageTag = u16;

pub struct BlockStorage {
    tag: StorageTag,
}

impl BlockStorage {
    //TODO: Error handling
    pub async fn new(tag: StorageTag) -> Self {
        let inner_path = format!("{}", tag);
        let path = Path::new(&inner_path);

        if !path.exists() {
            tokio::fs::create_dir(inner_path).await.unwrap();
        }

        Self { tag }
    }

    pub async fn create_block(&self, block_id: u64, data: &[u8]) {
        let file = OpenOptions::new()
            .write(true)
            .read(false)
            .create(true)
            .open(format!("{}/{}", self.tag, block_id))
            .await
            .unwrap();
        self.write_block(file, data).await;
    }

    pub async fn read_block(&self, block_id: u64) -> Vec<u8> {
        let file = OpenOptions::new()
            .write(false)
            .read(true)
            .open(format!("{}/{}", self.tag, block_id))
            .await
            .unwrap();
        let buffer_len = file.metadata().await.unwrap().len() as usize;
        let mut reader = BufReader::new(file);
        let mut buffer = vec![0; buffer_len];
        reader.read_exact(&mut buffer).await.unwrap();

        buffer
    }

    pub async fn update_block(&self, block_id: u64, data: &[u8]) {
        let file = OpenOptions::new()
            .write(true)
            .read(false)
            .open(format!("{}/{}", self.tag, block_id))
            .await
            .unwrap();
        self.write_block(file, data).await;
    }

    pub async fn delete_block(&self, block_id: u64) {
        tokio::fs::remove_file(format!("{}/{}", self.tag, block_id))
            .await
            .unwrap();
    }

    #[inline]
    async fn write_block(&self, file: File, data: &[u8]) {
        let mut writer = BufWriter::new(file);

        writer.seek(SeekFrom::Start(0)).await.unwrap();
        writer.write_all(data).await.unwrap();
        writer.flush().await.unwrap();
    }
}

#[cfg(test)]
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
        let block_id = 0;
        let first_record = b"Hello, World";
        let second_record = b"Hello, Pavel";

        let block_storage = BlockStorage::new(40000).await;
        block_storage.create_block(block_id, first_record).await;
        assert_eq!(
            first_record.as_slice(),
            block_storage.read_block(block_id).await.as_slice()
        );
        block_storage.update_block(block_id, second_record).await;
        assert_eq!(
            second_record.as_slice(),
            block_storage.read_block(block_id).await.as_slice()
        );
        block_storage.delete_block(block_id);
    }
}
