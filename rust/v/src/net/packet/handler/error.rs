use crate::net::packet::error::{PacketBuildError, PacketReadWriteError, PacketValidationError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PacketHandlerError {
    #[error("Packet failed to build in handler layer")]
    PacketHandlerBuildError(#[from] PacketBuildError),

    #[error("Packet invalid in handler layer")]
    PacketHandlerValidationError(#[from] PacketValidationError),

    #[error("Packet read/write error in handler layer")]
    PacketHandlerReadWriteError(#[from] PacketReadWriteError),

    #[error("Packet generic error in handler layer")]
    PacketHandlerGenericError(#[from] PacketGenericError),
}

#[derive(Debug, Error)]
pub enum PacketGenericError {
    #[error("Authentication failed in handler layer")]
    AuthenticationError,

    #[error("TOS failed in handler layer")]
    TOSError,
}
