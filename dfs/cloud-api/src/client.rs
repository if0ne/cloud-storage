use async_trait::async_trait;
use std::ops::Range;
use tokio::io::{AsyncReadExt, AsyncWrite};

#[async_trait]
pub trait CloudClient {
    async fn make_dir(&mut self, path: impl AsRef<std::path::Path> + Send);

    async fn create_small_file(&mut self, path: impl AsRef<std::path::Path> + Send + Sync);

    async fn read_small_file(&mut self, path: impl AsRef<std::path::Path> + Send);

    async fn commit_to_small_file(
        &mut self,
        path: impl AsRef<std::path::Path> + Send,
        data: impl AsRef<[u8]> + Send,
    );
}
