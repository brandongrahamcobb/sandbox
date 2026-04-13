use thiserror::Error;

use std::time::SystemTimeError;

#[derive(Debug, Error)]
pub enum NetworkError {
    #[error("Unsupported upcode error: {0}")]
    UnsupportedOpcodeError(i16),

    #[error("System Time Converstion Error: {0}")]
    SystemTimeError(SystemTimeError),

    // #[error("Infrastructure IO layer error: {0}")]
    // HandlerFailed(&'static str),
    #[error("Network handler error: {0}")]
    Io(#[from] std::io::Error),
}
