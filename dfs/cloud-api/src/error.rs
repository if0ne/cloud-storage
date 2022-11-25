use serde::{Deserialize, Serialize};
use thiserror::Error;
use tonic_error::TonicError;

#[derive(Debug, Error, TonicError, Serialize, Deserialize)]
pub enum BlockStorageError {
    #[error("fail to create block {0}")]
    CreateBlockError(String),
    #[error("block {0} was not found")]
    BlockNotFound(String),
    #[error("got wrong uuid format {0}")]
    WrongUuid(String),
    #[error("fail to read block {0}")]
    ReadBlockError(String),
    #[error("fail to update block {0}")]
    UpdateBlockError(String),
    #[error("fail to delete block {0}")]
    DeleteBlockError(String),
}
