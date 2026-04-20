use thiserror::Error;

use crate::{db::error::DatabaseError, net::error::NetworkError};

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("Config error in runtime layer")]
    RuntimeConfigError,

    #[error("Network error in runtime layer")]
    RuntimeNetworkError(#[from] NetworkError),

    #[error("Generic error in runtime layer")]
    GenericRuntimeError,

    #[error("Database error in runtime layer")]
    RuntimeDatabaseError(#[from] DatabaseError),

    #[error("Concurrency join error in runtime layer")]
    RuntimeJoinError,
}
