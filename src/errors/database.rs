use sea_orm::DbErr;
use thiserror::Error;

#[derive(Error, Debug)]

pub enum DatabaseError {
    #[error("{0}")]
    OperationFailed(String),

    #[error(transparent)]
    SqlxError(#[from] DbErr),

    #[error("failed to connect to database due to {0}")]
    ConnectionFailed(String),

    #[error("Invalid database config:{0}")]
    InvalidConfig(String),
}
