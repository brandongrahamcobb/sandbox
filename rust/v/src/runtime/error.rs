use thiserror::Error;

use crate::{db::error::DatabaseError, net::error::NetworkError};

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("Config error in runtime layer")]
    RuntimeConfigError,

    #[error("Network error in runtime layer")]
    RuntimeNetworkError(#[from] NetworkError),

    #[error("Generic error in runtime layer: {0}")]
    GenericRuntimeError(String),

    #[error("Database error in runtime layer")]
    RuntimeDatabaseError(#[from] DatabaseError),

    #[error("Concurrency join error in runtime layer")]
    RuntimeJoinError,

    #[error("Unexpected end of output in runtime layer")]
    RuntimeUnexpectedOf(#[from] std::io::Error),

    #[error("Failed to connect to server in runtime layer")]
    FailedServerConnection(#[from] RuntimeServerConnectionError),

    #[error("Failed to create relay in runtime layer")]
    FailedRelayCreation(#[from] RuntimeRelayCreationError),
}

#[derive(Debug, Error)]
pub enum RuntimeServerConnectionError {
    #[error("Failed login server connection: {0}")]
    FailedLoginServerConnection(String),

    #[error("Failed world server connection: {0}")]
    FailedWorldServerConnection(String),
}

#[derive(Debug, Error)]
pub enum RuntimeRelayCreationError {
    #[error("Failed to create login relay: {0}")]
    FailedLoginRelayCreation(String),

    #[error("Failed to create world relay: {0}")]
    FailedWorldRelayCreation(String),
}
