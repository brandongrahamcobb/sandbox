use thiserror::Error;

#[derive(Debug, Error)]
pub enum WorldError {
    #[error("Requested world was not found in world layer: {0}")]
    NotFound(u8),
}
