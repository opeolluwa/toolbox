use thiserror::Error;

use crate::errors::{database::DatabaseError, file_system::FileSystemError};

#[derive(Error, Debug)]

pub enum AppError {
    #[error("{0}")]
    OperationFailed(String),

    #[error(transparent)]
    FileSystemError(#[from] FileSystemError),

    #[error("$HOME path not found")]
    HomeDirNotFound,

    #[error(transparent)]
    DatabaseError(#[from] DatabaseError),
}
