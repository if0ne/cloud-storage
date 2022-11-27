use async_trait::async_trait;
use std::ops::Range;
use tokio::io::{AsyncReadExt, AsyncWrite};

#[async_trait]
pub trait CloudClient {
    async fn make_dir(&mut self, path: impl AsRef<std::path::Path> + Send);

    async fn create_small_file(&mut self, path: impl AsRef<std::path::Path> + Send + Sync);
    async fn create_small_file_from_stream(
        &mut self,
        path: impl AsRef<std::path::Path> + Send,
    );

    async fn create_large_file(&mut self, path: impl AsRef<std::path::Path> + Send);
    async fn create_large_file_from_stream(
        &mut self,
        path: impl AsRef<std::path::Path> + Send,
    );

    async fn read_small_file(&mut self, path: impl AsRef<std::path::Path> + Send);
    async fn read_large_file(&mut self, path: impl AsRef<std::path::Path> + Send, blocks: Range<usize>);

    async fn commit_to_small_file(&mut self, path: impl AsRef<std::path::Path> + Send, data: impl AsRef<[u8]> + Send);
    async fn commit_to_small_file_from_stream(
        &mut self,
        path: impl AsRef<std::path::Path> + Send,
        stream: impl AsyncReadExt + AsyncWrite + Send,
    );
}
