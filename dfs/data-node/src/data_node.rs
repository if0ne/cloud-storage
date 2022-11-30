use std::io::SeekFrom;

use crate::data_node_info::DataNodeInfo;
use cloud_api::error::DataNodeError;
use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt, BufReader, BufWriter};
use uuid::Uuid;

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

    pub async fn read_block(&self, block_id: Uuid) -> Result<Vec<u8>, DataNodeError> {
        let path = self
            .data_node_info
            .found_block(block_id.as_u128().to_string())
            .await?;
        let file = OpenOptions::new()
            .write(false)
            .read(true)
            .open(path)
            .await
            .map_err(|err| DataNodeError::ReadBlockError(err.to_string()))?;
        let buffer_len = file
            .metadata()
            .await
            .map_err(|err| DataNodeError::ReadBlockError(err.to_string()))?
            .len() as usize;
        let mut reader = BufReader::new(file);
        let mut buffer = vec![0; buffer_len];
        reader
            .read_exact(&mut buffer)
            .await
            .map_err(|err| DataNodeError::ReadBlockError(err.to_string()))?;

        Ok(buffer)
    }

    pub async fn update_block(&self, block_id: Uuid, data: &[u8]) -> Result<(), DataNodeError> {
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
        for disk in self.data_node_info.disks {
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
        let first_record = b"Hello, World";
        let second_record = b"Hello, Pavel";

        let data_node_info = DataNodeInfo::new(Config {
            main_server_address: "[::1]:8000".to_string(),
            port: 40000,
            block_size: 32,
            disk_space: None,
            working_directory: "test_dir".to_string(),
        })
        .await;

        let data_node = DataNode::new(data_node_info).await.unwrap();
        let uuid = data_node.create_block().await.unwrap();
        assert_eq!(
            first_record.as_slice(),
            data_node.read_block(uuid).await.as_slice()
        );
        data_node.update_block(uuid, second_record).await.unwrap();
        assert_eq!(
            second_record.as_slice(),
            data_node.read_block(uuid).await.as_slice()
        );
        data_node.delete_block(uuid).await.unwrap();
    }
}
