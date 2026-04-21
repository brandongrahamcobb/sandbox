use thiserror::Error;

#[derive(Debug, Error)]
pub enum PacketIOError {
    #[error("Failed to read packet in packet io layer: {0}")]
    ReadError(String),

    #[error("Failed to write to packet in packet io layer: {0}")]
    WriteError(String),
}

#[derive(Debug, Error)]
pub enum GenericPacketIOError {
    #[error("Packet io failed in io layer")]
    GenericError(#[from] PacketIOError),
}
