use thiserror::Error;

#[derive(Debug, Error)]
pub enum LoginError {
    #[error("Account is banned error in login packet handler layer")]
    Banned,

    #[error("Terms of Service denied error in login packet handler layer")]
    DeniedTOS,

    #[error("Incorrect credentials error in login packet handler layer")]
    InvalidCredentials,

    #[error("Terms of Service has not been aceepted error in login packet handler layer")]
    PendingTOS,

    #[error("Account is playing error in login packet handler layer")]
    Playing,

    #[error("Unhandled error in login packet handler layer")]
    UnexpectedError,
}
