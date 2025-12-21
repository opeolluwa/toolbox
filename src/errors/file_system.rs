use thiserror::Error;
use confy::ConfyError;

#[derive(Error, Debug)]
pub enum FileSystemError {
    #[error("{0}")]
    OperationError(String),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    ConfyError(#[from] ConfyError)
}
