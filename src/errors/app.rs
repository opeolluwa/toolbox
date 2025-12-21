use thiserror::Error;

use crate::errors::file_system::FileSystemError;

#[derive(Error, Debug)]

pub enum AppError {
    #[error("{0}")]
    OperationFailed(String),
    #[error(transparent)]
    FileSystemError(#[from] FileSystemError),
    #[error("$HOME path not found")]
    HomeDirNotFound,
}
