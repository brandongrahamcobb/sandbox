use thiserror::Error;

#[derive(Debug, Error)]
pub enum BuildError {
    #[error("Failed to build terms of service prompt packet in packet layer")]
    PromptTOSError,

    #[error("Failed to build successful credentials packet in packet layer")]
    SuccessfulLoginError,

    #[error("Failed to build failed credentials packet in packet layer")]
    FailedLoginError,
}
