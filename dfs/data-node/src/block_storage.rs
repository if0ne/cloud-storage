use std::path::Path;
use std::sync::atomic::Ordering;

use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader, BufWriter};

pub type StorageTag = u16;

//TODO: id type
pub struct BlockStorage {
    tag: StorageTag,
    id: std::sync::atomic::AtomicUsize,
}

impl BlockStorage {
    //TODO: Error handling
    pub async fn new(tag: StorageTag) -> Self {
        let inner_path = format!("{}", tag);
        let path = Path::new(&inner_path);

        if path.exists() {
            let file_count = std::fs::read_dir(path).unwrap().count();
            Self {
                tag,
                id: file_count.into(),
            }
        } else {
            tokio::fs::create_dir(inner_path).await.unwrap();
            Self { tag, id: 0.into() }
        }
    }

    pub async fn create_block(&self, data: &[u8]) {
        let old_id = self.id.load(Ordering::Acquire);
        self.id.store(old_id + 1, Ordering::Release);

        let file = File::create(format!("{}/{}", self.tag, old_id))
            .await
            .unwrap();
        let mut writer = BufWriter::new(file);

        writer.write_all(data).await.unwrap();
        writer.flush().await.unwrap();
    }

    pub async fn read_block(&self, block_id: usize) -> Vec<u8> {
        let file = File::open(format!("{}/{}", self.tag, block_id))
            .await
            .unwrap();
        let buffer_len = file.metadata().await.unwrap().len() as usize;
        let mut reader = BufReader::new(file);
        let mut buffer = vec![0; buffer_len];
        reader.read_exact(&mut buffer).await.unwrap();

        buffer
    }
}
