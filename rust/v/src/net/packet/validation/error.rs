use thiserror::Error;

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("Invalid packet header in packet validation layer")]
    InvalidHeader,

    #[error("Invalid packet length in packet validation layer: {0}")]
    InvalidPacketLength(i16),

    #[error("Empty packet in packet validation layer")]
    EmptyPacket,
}
