use thiserror::Error;

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("Handler error: {0}")]
    Handler(String),

    #[error("Client disconnected")]
    ClientDisconnected,

    #[error("Server socket error: {0}")]
    Io(#[from] std::io::Error),
}
