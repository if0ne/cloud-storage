use serde::{Deserialize, Serialize};
use std::fmt::Formatter;

#[derive(Debug, Serialize, Deserialize)]
pub enum BlockStorageError {
    CreateBlockError(String),
    BlockNotFound(String),
    WrongUuid(String),
    ReadBlockError(String),
    UpdateBlockError(String),
    DeleteBlockError(String),
}

impl std::fmt::Display for BlockStorageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BlockStorageError::CreateBlockError(str) => {
                write!(f, "fail to create block {0}", str)
            }
            BlockStorageError::BlockNotFound(str) => {
                write!(f, "block {0} was not found", str)
            }
            BlockStorageError::WrongUuid(str) => {
                write!(f, "got wrong uuid format {0}", str)
            }
            BlockStorageError::ReadBlockError(str) => {
                write!(f, "fail to read block {0}", str)
            }
            BlockStorageError::UpdateBlockError(str) => {
                write!(f, "fail to update block {0}", str)
            }
            BlockStorageError::DeleteBlockError(str) => {
                write!(f, "fail to delete block {0}", str)
            }
        }
    }
}

impl From<BlockStorageError> for tonic::Status {
    fn from(error: BlockStorageError) -> Self {
        tonic::Status::internal(error.to_string())
    }
}
