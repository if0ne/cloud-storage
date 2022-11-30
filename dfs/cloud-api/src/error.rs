use serde::{Deserialize, Serialize};
use std::fmt::Formatter;

#[derive(Debug, Serialize, Deserialize)]
pub enum DataNodeError {
    CreateBlockError(String),
    BlockNotFound(String),
    WrongUuid(String),
    ReadBlockError(String),
    UpdateBlockError(String),
    DeleteBlockError(String),
    NoSpace,
}

impl std::fmt::Display for DataNodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DataNodeError::CreateBlockError(str) => {
                write!(f, "Fail to create block {0}", str)
            }
            DataNodeError::BlockNotFound(str) => {
                write!(f, "Block {0} was not found", str)
            }
            DataNodeError::WrongUuid(str) => {
                write!(f, "Got wrong uuid format {0}", str)
            }
            DataNodeError::ReadBlockError(str) => {
                write!(f, "Fail to read block {0}", str)
            }
            DataNodeError::UpdateBlockError(str) => {
                write!(f, "Fail to update block {0}", str)
            }
            DataNodeError::DeleteBlockError(str) => {
                write!(f, "Fail to delete block {0}", str)
            }
            DataNodeError::NoSpace => {
                write!(f, "No space")
            }
        }
    }
}

impl From<DataNodeError> for tonic::Status {
    fn from(error: DataNodeError) -> Self {
        tonic::Status::internal(error.to_string())
    }
}
