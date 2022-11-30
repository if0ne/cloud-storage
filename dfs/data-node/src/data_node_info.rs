use cloud_api::error::DataNodeError;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::io::SeekFrom;
use std::path::{Path, PathBuf};
use sysinfo::{DiskExt, SystemExt};
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt};

use crate::config::Config;
use crate::disk_stats::DiskStats;

#[allow(dead_code)]
#[derive(Debug)]
pub struct DataNodeInfo {
    pub port: u16,
    pub(crate) working_directory: Box<Path>,
    pub(crate) block_size: u32,
    pub(crate) total_space: u64,
    pub(crate) disks: Vec<DiskStats>,
}

impl DataNodeInfo {
    pub async fn new(config: Config) -> Self {
        let path = format!("{}_{}", config.working_directory, config.block_size);
        let working_directory = Box::from(Path::new(&path));
        let disks = Self::get_disks(config.block_size, &working_directory);
        let total_space = disks
            .iter()
            .fold(0, |space, disk| space + disk.available_space);

        //TODO: Rewrite smth
        let used_space = {
            let mut used_space = 0;
            for disk in &disks {
                used_space += *disk.used_space.read().await
            }

            used_space
        };

        let status = Self::check_config(&config, &path).await;
        if let Ok(status) = status {
            if !status && used_space != 0 {
                panic!("Wrong block size")
            }
        }

        Self::save_state(&config, &path).await.unwrap();

        Self {
            port: config.port,
            working_directory,
            block_size: config.block_size,
            total_space,
            disks,
        }
    }

    pub(crate) async fn get_disk_for_new_block(&self) -> Result<&DiskStats, DataNodeError> {
        for disk in &self.disks {
            let mut writer = disk.used_space.write().await;
            if *writer + (self.block_size as u64) <= self.total_space {
                *writer += self.block_size as u64;
                return Ok(disk);
            }
        }

        Err(DataNodeError::NoSpace)
    }

    pub(crate) async fn found_block(
        &self,
        uuid: impl AsRef<Path>,
    ) -> Result<PathBuf, DataNodeError> {
        for disk in &self.disks {
            let path = disk.mount.join(&self.working_directory).join(&uuid);
            if path.exists() {
                return Ok(path);
            }
        }

        Err(DataNodeError::BlockNotFound(
            uuid.as_ref().to_string_lossy().to_string(),
        ))
    }

    async fn save_state(config: &Config, suffix: &str) -> std::io::Result<()> {
        let mut hasher = DefaultHasher::new();
        ((config.block_size as u64 * 2) >> 6).hash(&mut hasher);

        let hash = hasher.finish();
        let path = format!(".data_node_info_{}", suffix);
        let mut file = tokio::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(&path)
            .await?;
        file.seek(SeekFrom::Start(0)).await?;
        file.write_u64(hash).await?;

        Ok(())
    }

    async fn check_config(
        config: &Config,
        suffix: &str,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut hasher = DefaultHasher::new();
        ((config.block_size as u64 * 2) >> 6).hash(&mut hasher);

        let hash = hasher.finish();
        let path = format!(".data_node_info_{}", suffix);
        let mut file = tokio::fs::OpenOptions::new().read(true).open(&path).await?;
        let read = file.read_u64().await?;

        Ok(hash == read)
    }

    fn get_disks(block_size: u32, working_directory: impl AsRef<Path>) -> Vec<DiskStats> {
        let mut system = sysinfo::System::new_all();
        system.refresh_all();
        system.sort_disks_by(|l_disk, r_disk| r_disk.available_space().cmp(&l_disk.total_space()));

        system
            .disks()
            .iter()
            .filter_map(|disk| {
                DiskStats::new(
                    disk.total_space(),
                    disk.available_space(),
                    block_size,
                    disk.mount_point(),
                    &working_directory,
                )
            })
            .collect()
    }
}
