use thiserror::Error;

#[derive(Debug, Error)]
pub enum LoginPacketHandlerError {
    #[error("Incorrect credentials provided in login packet handler layer")]
    InvalidCredentials,

    #[error("Terms of Service denied in login packet handler layer")]
    DeniedTOS,

    #[error("Unhandled error in login packet handler layer")]
    UnexpectedError,
}

#[derive(Debug, Error)]
pub enum GenericLoginPacketHandlerError {
    #[error("Login packet handler failed in login packet handler layer")]
    GenericError(#[from] LoginPacketHandlerError),
}
