use crate::db::models::error::ModelError;
use diesel;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("Failed to connect to database in database layer")]
    DatabaseConnectionError,

    #[error("Invalid database url error: {0}")]
    InvalidDatabaseUrlError(String),

    #[error("Database failed for model in database layer")]
    ModelError(#[from] ModelError),

    #[error("Database pool failed in database layer")]
    DatabasePoolError(#[from] diesel::result::Error),
}
