use thiserror::Error;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("Connection error in the database layer")]
    DatabaseConnectionError(#[from] InvalidDatabaseIoError::InvalidDatabaseUrlError),
}

#[derive(Debug, Error)]
pub enum DatabaseIoError {
    #[error("Invalid database url error: {0}")]
    InvalidDatabaseUrlError(String),
}
