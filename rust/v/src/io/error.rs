use thiserror::Error;

#[derive(Debug, Error)]
pub enum PacketError {
    #[error("Packet read/write error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Invalid packet header")]
    InvalidHeader,

    #[error("Invalid packet length: {0}")]
    InvalidPacketLength(i16),

    #[error("Empty packet.")]
    EmptyPacket,
}
