use std::time::SystemTimeError;

use crate::db::error::DatabaseError;
use crate::net::packet::error::PacketError;
use crate::runtime::error::SessionError;
use config::ConfigError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NetworkError {
    #[error("Unsupported upcode error in network layer: {0}")]
    UnsupportedOpcodeError(i16),

    #[error("Packet error in network layer")]
    PacketError(#[from] PacketError),

    #[error("Database error in network layer")]
    DatabaseError(#[from] DatabaseError),

    #[error("Session error in network layer")]
    SessionError(#[from] SessionError),

    #[error("Config error in network layer")]
    ConfigError(#[from] ConfigError),

    #[error("System time error in network layer")]
    SystemTimeError(#[from] SystemTimeError),
}
