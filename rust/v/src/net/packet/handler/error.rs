use crate::net::packet::handler::login::error::GenericLoginPacketHandlerError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PacketHandlerError {
    #[error("Login packet failed in handler layer")]
    LoginError(#[from] GenerticLoginPacketHandlerError),
}

#[derive(Debug, Error)]
pub enum GenericPacketHandlerError {
    #[error("Packet handler failed in handler layer")]
    GenericError(#[from] PacketHandlerError),
}
