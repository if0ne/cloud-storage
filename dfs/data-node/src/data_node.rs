use cloud_api::error::DataNodeError;
use futures::TryFutureExt;
use std::io::SeekFrom;
use std::ops::Range;
use std::path::{Path, PathBuf};
use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt, BufReader, BufWriter};
use uuid::Uuid;

use super::data_node_info::DataNodeInfo;

pub struct DataNode {
    data_node_info: DataNodeInfo,
}

impl DataNode {
    pub async fn new(data_node_info: DataNodeInfo) -> std::io::Result<Self> {
        for disk in &data_node_info.disks {
            let path = disk.mount.join(&data_node_info.working_directory);
            if !path.exists() {
                tokio::fs::create_dir(path).await?;
            }
        }

        Ok(Self { data_node_info })
    }

    pub async fn create_block(&self) -> Result<Uuid, DataNodeError> {
        let uuid = Uuid::new_v4();
        let disk = self.data_node_info.get_disk_for_new_block().await?;
        let _ = OpenOptions::new()
            .write(true)
            .read(false)
            .create(true)
            .open(
                disk.mount
                    .join(&self.data_node_info.working_directory)
                    .join(uuid.as_u128().to_string()),
            )
            .await
            .map_err(|err| DataNodeError::CreateBlockError(err.to_string()))?;

        Ok(uuid)
    }

    pub async fn read_block(
        &self,
        block: impl AsRef<Path>,
        bytes: Range<usize>,
    ) -> Result<Vec<u8>, DataNodeError> {
        let file = OpenOptions::new()
            .write(false)
            .read(true)
            .open(block)
            .await
            .map_err(|err| DataNodeError::ReadBlockError(err.to_string()))?;
        let mut reader = BufReader::new(file);
        let mut buffer = vec![0; bytes.len()];
        reader
            .seek(SeekFrom::Start(bytes.start as u64))
            .map_err(|err| DataNodeError::ReadBlockError(err.to_string()))
            .await?;
        reader
            .read_exact(&mut buffer)
            .await
            .map_err(|err| DataNodeError::ReadBlockError(err.to_string()))?;
        reader
            .flush()
            .await
            .map_err(|err| DataNodeError::ReadBlockError(err.to_string()))?;

        Ok(buffer)
    }

    pub async fn update_block(&self, block_id: Uuid, data: &[u8]) -> Result<(), DataNodeError> {
        if data.len() > self.data_node_info.block_size {
            return Err(DataNodeError::BlockOverflow(
                self.data_node_info.block_size,
                data.len(),
            ));
        }

        let path = self
            .data_node_info
            .found_block(block_id.as_u128().to_string())
            .await?;
        let file = OpenOptions::new()
            .write(true)
            .read(false)
            .open(path)
            .await
            .map_err(|_| DataNodeError::UpdateBlockError(block_id.to_string()))?;
        self.write_block(file, data).await
    }

    pub async fn delete_block(&self, block_id: Uuid) -> Result<(), DataNodeError> {
        let path = self
            .data_node_info
            .found_block(block_id.as_u128().to_string())
            .await?;
        tokio::fs::remove_file(path)
            .await
            .map_err(|_| DataNodeError::DeleteBlockError(block_id.to_string()))
    }

    pub async fn get_block_info(&self, block_id: Uuid) -> Result<(PathBuf, usize), DataNodeError> {
        let path = self
            .data_node_info
            .found_block(block_id.as_u128().to_string())
            .await?;
        let buffer_len = path
            .metadata()
            .map_err(|err| DataNodeError::ReadBlockError(err.to_string()))?
            .len() as usize;

        Ok((path, buffer_len))
    }

    pub fn get_data_node_info(&self) -> &DataNodeInfo {
        &self.data_node_info
    }

    #[inline]
    async fn write_block(&self, file: File, data: &[u8]) -> Result<(), DataNodeError> {
        let mut writer = BufWriter::new(file);

        writer
            .seek(SeekFrom::Start(0))
            .await
            .map_err(|err| DataNodeError::UpdateBlockError(err.to_string()))?;
        writer
            .write_all(data)
            .await
            .map_err(|err| DataNodeError::UpdateBlockError(err.to_string()))?;
        writer
            .flush()
            .await
            .map_err(|err| DataNodeError::UpdateBlockError(err.to_string()))?;

        Ok(())
    }
}

#[cfg(any(test, bench))]
impl Drop for DataNode {
    fn drop(&mut self) {
        for disk in &self.data_node_info.disks {
            let _ =
                std::fs::remove_dir_all(disk.mount.join(&self.data_node_info.working_directory));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;

    #[tokio::test]
    async fn test_block_crud() {
        let message = b"Hello, Pavel";

        let data_node_info = DataNodeInfo::new(Config {
            main_server_address: "http://[::1]:8000".to_string(),
            self_address: "http://[::1]".to_string(),
            port: 40000,
            block_size: 32,
            disk_space: None,
            working_directory: "test_dir".to_string(),
            read_buffer: 8,
        })
        .await;
        let buffer_size = data_node_info.read_buffer;

        let data_node = DataNode::new(data_node_info).await.unwrap();
        let uuid = data_node.create_block().await.unwrap();

        data_node.update_block(uuid, message).await.unwrap();
        let (path, len) = data_node.get_block_info(uuid).await.unwrap();
        let chunk_count = len / buffer_size;
        let last_chunk = len - chunk_count * buffer_size;

        let mut read_message = vec![];
        for i in 0..(chunk_count + 1) {
            let bytes = if i == chunk_count {
                if last_chunk == 0 {
                    break;
                }
                (i * buffer_size)..(i * buffer_size + last_chunk)
            } else {
                (i * buffer_size)..((i + 1) * buffer_size)
            };
            let read = data_node.read_block(&path, bytes).await.unwrap();
            read_message = [read_message, read].concat();
        }

        assert_eq!(message.as_slice(), read_message.as_slice());
        data_node.delete_block(uuid).await.unwrap();
    }

    #[tokio::test]
    async fn test_multi_read_access() {
        let message = std::iter::repeat(*b"Hello, Pavel ")
            .take(4096)
            .flatten()
            .collect::<Vec<_>>();
        assert_eq!(message.len(), 13 * 4096);

        let data_node_info = DataNodeInfo::new(Config {
            main_server_address: "http://[::1]:8000".to_string(),
            self_address: "http://[::1]".to_string(),
            port: 40000,
            block_size: 65536,
            disk_space: None,
            working_directory: "test_dir".to_string(),
            read_buffer: 1000,
        })
        .await;
        let buffer_size = data_node_info.read_buffer;

        let data_node = DataNode::new(data_node_info).await.unwrap();
        let uuid = data_node.create_block().await.unwrap();

        data_node.update_block(uuid, &message).await.unwrap();
        let (path, len) = data_node.get_block_info(uuid).await.unwrap();
        let chunk_count = len / buffer_size;
        let last_chunk = len - chunk_count * buffer_size;
        let mut futures = vec![];
        for _ in 0..100 {
            futures.push(async {
                let mut read_message = vec![];
                for i in 0..(chunk_count + 1) {
                    let bytes = if i == chunk_count {
                        if last_chunk == 0 {
                            break;
                        }
                        (i * buffer_size)..(i * buffer_size + last_chunk)
                    } else {
                        (i * buffer_size)..((i + 1) * buffer_size)
                    };
                    let read = data_node.read_block(&path, bytes).await.unwrap();
                    read_message = [read_message, read].concat();
                }
                read_message
            })
        }
        let results = futures::future::join_all(futures).await;
        for i in results {
            assert_eq!(message.as_slice(), i.as_slice());
        }

        data_node.delete_block(uuid).await.unwrap();
    }
}
