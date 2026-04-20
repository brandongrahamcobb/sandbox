use thiserror::Error;

use crate::net::packet::error::{GenericPacketError, PacketReadWriteError};
use crate::net::packet::handler::error::PacketHandlerError;
use std::time::SystemTimeError;

#[derive(Debug, Error)]
pub enum NetworkError {
    #[error("Unsupported upcode error in network layer: {0}")]
    UnsupportedOpcodeError(i16),

    #[error("System Time conversion error in network layer")]
    SystemTimeError(#[from] SystemTimeError),

    #[error("Packet handler error in network layer")]
    NetworkPacketHandlerError(#[from] PacketHandlerError),

    #[error("Generic packet error in network layer")]
    NetworkGenericPacketError(#[from] GenericPacketError),

    #[error("Handshake error in network layer")]
    NetworkHandshakeError,

    #[error("Packet read/write error in network layer")]
    NetworkPacketReadWriteError(#[from] PacketReadWriteError),

    #[error("Client disconnect in network layer")]
    ClientDisconnectError,
}
