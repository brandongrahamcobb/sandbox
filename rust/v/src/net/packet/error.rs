use crate::net::packet::build::error::GenericPacketBuildError;
use crate::net::packet::handler::error::GenericPacketHandlerError;
use crate::net::packet::io::error::GenericPacketIOError;
use crate::net::packet::validation::error::GenericPacketValidationError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PacketError {
    #[error("Packet failed to build in packet layer")]
    BuildError(#[from] GenericPacketBuildError),

    #[error("Invalid packet in packet layer")]
    ValidationError(#[from] GenericPacketValidationError),

    #[error("Packet io error in packet layer")]
    IOError(#[from] GenericPacketIOError),

    #[error("Packet handler error in packet layer")]
    HandlerError(#[from] GenericPacketHandlerError),
}

#[derive(Debug, Error)]
pub enum GenericPacketError {
    #[error("Packet failed in packet layer")]
    GenericError(#[from] PacketError),
}
