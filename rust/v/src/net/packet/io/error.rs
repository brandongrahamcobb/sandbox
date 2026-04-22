use thiserror::Error;

#[derive(Debug, Error)]
pub enum IOError {
    #[error("Failed to read packet in packet io layer")]
    ReadError(std::io::Error),

    #[error("Failed to write to packet in packet io layer")]
    WriteError(std::io::Error),
}
