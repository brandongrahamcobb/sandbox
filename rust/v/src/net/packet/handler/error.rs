use crate::net::packet::handler::core::login::error::LoginError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HandlerError {
    #[error("Login packet failed in handler layer")]
    LoginError(#[from] LoginError),
}
