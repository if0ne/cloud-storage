use serde::{Deserialize, Serialize};
use std::fmt::Formatter;
use tonic::{Code, Status};

const CUSTOM_ERROR: &str = "x-custom-tonic-error";

#[derive(Debug, Serialize, Deserialize)]
pub enum ConvertError {
    TypeToStatus(String),
    StatusToType(String),
}

impl std::fmt::Display for ConvertError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ConvertError::TypeToStatus(err) => {
                write!(f, "{}", err)
            }
            ConvertError::StatusToType(err) => {
                write!(f, "{}", err)
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DataNodeError {
    CreateBlockError(String),
    BlockNotFound(String),
    WrongUuid(String),
    ReadBlockError(String),
    UpdateBlockError(String),
    DeleteBlockError(String),
    NoSpace,
    BlockOverflow(usize, usize),
}

crate::impl_converter!(DataNodeError);

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
            DataNodeError::BlockOverflow(block_size, buffer_size) => {
                write!(
                    f,
                    "Trying to write {} bytes in block of {} bytes size",
                    buffer_size, block_size
                )
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RegistryError {
    WrongBlockSize(usize, usize, usize),
}

crate::impl_converter!(RegistryError);

impl std::fmt::Display for RegistryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RegistryError::WrongBlockSize(got, small, large) => {
                write!(
                    f,
                    "Got block size {} bytes, but server can handle in block size with {} or {} bytes.",
                    got, small, large
                )
            }
        }
    }
}

#[macro_export]
macro_rules! impl_converter {
    ($error:ty) => {
        impl From<$error> for tonic::Status {
            fn from(error: $error) -> Self {
                let mut status = tonic::Status::internal(error.to_string());
                status.metadata_mut().insert(
                    CUSTOM_ERROR,
                    serde_json::to_string(&error).unwrap().parse().unwrap(),
                );

                status
            }
        }

        /*impl TryFrom<$error> for tonic::Status {
            type Error = ConvertError;

            fn try_from(error: $error) -> Result<Self, Self::Error> {
                let mut status = tonic::Status::internal(error.to_string());
                status.metadata_mut().insert(
                    CUSTOM_ERROR,
                    serde_json::to_string(&error)
                        .map_err(|err| ConvertError::TypeToStatus(err.to_string()))?
                        .parse()
                        .map_err(|err: tonic::metadata::errors::InvalidMetadataValue| {
                            ConvertError::TypeToStatus(err.to_string())
                        })?,
                );

                Ok(status)
            }
        }*/

        impl TryFrom<Status> for $error {
            type Error = ConvertError;

            fn try_from(err: Status) -> Result<Self, Self::Error> {
                match err.code() {
                    Code::Internal => {
                        if let Some(err) = err.metadata().get(CUSTOM_ERROR) {
                            let err = serde_json::from_str(
                                err.to_str()
                                    .map_err(|err| ConvertError::StatusToType(err.to_string()))?,
                            )
                            .map_err(|err| ConvertError::StatusToType(err.to_string()))?;
                            return Ok(err);
                        } else {
                            Err(ConvertError::StatusToType(
                                "Unknown internal error".to_string(),
                            ))
                        }
                    }
                    _ => Err(ConvertError::StatusToType("Other rpc error".to_string())),
                }
            }
        }
    };
}
