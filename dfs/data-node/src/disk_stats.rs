use std::path::Path;
use tokio::sync::RwLock;

#[allow(dead_code)]
#[derive(Debug)]
pub(crate) struct DiskStats {
    pub(crate) available_space: u64,
    pub(crate) used_space: RwLock<u64>,
    pub(crate) mount: Box<Path>,
}

impl DiskStats {
    pub(crate) fn new(
        total_space: u64,
        available_space: u64,
        block_size: u32,
        mount: impl AsRef<Path>,
        working_directory: impl AsRef<Path>,
    ) -> Option<Self> {
        let used_space = if let Ok(dir) = std::fs::read_dir(mount.as_ref().join(working_directory))
        {
            (dir.into_iter().count() as u64) * (block_size as u64)
        } else {
            0
        };

        if let Ok(metadata) = std::fs::metadata(&mount) {
            if metadata.permissions().readonly() {
                return None;
            }
        } else {
            return None;
        }

        Some(Self {
            available_space: total_space - available_space + used_space,
            used_space: RwLock::new(used_space),
            mount: Box::from(mount.as_ref()),
        })
    }
}
