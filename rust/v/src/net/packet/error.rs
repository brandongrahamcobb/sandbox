use thiserror::Error;

#[derive(Debug, Error)]
pub enum PacketValidationError {
    #[error("Invalid packet header in packet layer")]
    InvalidHeader,

    #[error("Invalid packet length in packet layer: {0}")]
    InvalidPacketLength(i16),

    #[error("Empty packet in packet layer")]
    EmptyPacket,
}

#[derive(Debug, Error)]
pub enum PacketReadWriteError {
    #[error("Read packet failed in packet layer")]
    PacketReadError,

    #[error("Write packet failed in packet layer")]
    PacketWriteError,
}

#[derive(Debug, Error)]
pub enum PacketBuildError {
    #[error("Accept TOS packet failed to build in packet layer")]
    PacketBuildTOSError,

    #[error("Authentication packet failed to build in packet layer")]
    PacketBuildAuthenticationError,
}

#[derive(Debug, Error)]
pub enum GenericPacketError {
    #[error("Generic build error in packet layer")]
    PacketGenericBuildError(#[from] PacketBuildError),

    #[error("Generic read write error in packet layer")]
    PacketGenericReadWriteError(#[from] PacketReadWriteError),

    #[error("Generic validation error in packet layer")]
    PacketGenericValidationError(#[from] PacketValidationError),
}
