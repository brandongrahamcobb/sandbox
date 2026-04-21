use crate::net::packet::error::GenericPacketError;
use std::time::SystemTimeError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NetworkError {
    #[error("Unsupported upcode error in network layer: {0}")]
    UnsupportedOpcodeError(i16),

    #[error("System Time conversion error in network layer")]
    SystemTimeError(#[from] SystemTimeError),

    #[error("Packet handler error in network layer")]
    HandlerError(#[from] PacketHandlerError),

    #[error("Handshake error in network layer")]
    HandshakeError,

    #[error("Unexpected end of output in network layer")]
    UnexpectedOf(#[from] std::io::Error),

    #[error("Packet error in network layer")]
    PacketError(#[from] GenericPacketError),
}

#[derive(Debug, Error)]
pub enum GenericNetworkError {
    #[error("Network failed in network layer")]
    GenericError(#[from] NetworkError),
}
