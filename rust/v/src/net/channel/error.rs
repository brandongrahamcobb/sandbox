use thiserror::Error;

#[derive(Debug, Error)]
pub enum ChannelError {
    #[error("Requested channel was not found in channel layer: {0}")]
    NotFound(u8),
}
