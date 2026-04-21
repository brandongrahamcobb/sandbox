use thiserror::Error;

#[derive(Debug, Error)]
pub enum PacketBuildError {
    #[error("Failed to build terms of service prompt packet in packet layer")]
    PromptTOSError,

    #[error("Failed to build successful credentials packet in packet layer")]
    SuccessfulLoginError,

    #[error("Failed to build failed credentials packet in packet layer")]
    FailedLoginError,
}

#[derive(Debug, Error)]
pub enum GenericPacketBuildError {
    #[error("Packet build failed in build packet layer")]
    GenericError(#[from] PacketBuildError),
}
