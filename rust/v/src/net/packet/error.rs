use crate::net::packet::handler::error::HandlerError;
use crate::net::packet::io::error::IOError;
use crate::net::packet::validation::error::ValidationError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PacketError {
    #[error("Packet handler error in packet layer")]
    HandlerError(#[from] HandlerError),

    #[error("Packet io error in packet layer")]
    IOError(#[from] IOError),

    #[error("Invalid packet in packet layer")]
    ValidationError(#[from] ValidationError),
}
