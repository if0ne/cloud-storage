use async_trait::async_trait;
use std::ops::Range;
use tokio::io::{AsyncReadExt, AsyncWrite};

#[async_trait]
pub trait CloudClient {
    async fn make_dir(&self, path: impl AsRef<std::path::Path>);

    async fn create_small_file(&self, path: impl AsRef<std::path::Path>, data: impl AsRef<[u8]>);
    async fn create_small_file_from_stream(
        &self,
        path: impl AsRef<std::path::Path>,
        stream: impl AsyncReadExt + AsyncWrite,
    );

    async fn create_large_file(&self, path: impl AsRef<std::path::Path>, data: impl AsRef<[u8]>);
    async fn create_large_file_from_stream(
        &self,
        path: impl AsRef<std::path::Path>,
        stream: impl AsyncReadExt + AsyncWrite,
    );

    async fn read_small_file(
        &self,
        path: impl AsRef<std::path::Path>,
    );
    async fn read_large_file(
        &self,
        path: impl AsRef<std::path::Path>,
        blocks: Range<usize>,
    );

    async fn commit_to_small_file(&self, path: impl AsRef<std::path::Path>, data: impl AsRef<[u8]>);
    async fn commit_to_small_file_from_stream(
        &self,
        path: impl AsRef<std::path::Path>,
        stream: impl AsyncReadExt + AsyncWrite,
    );
}
