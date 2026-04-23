use crate::db::models::account::error::AccountError;
use crate::db::models::char::error::CharacterError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ModelError {
    #[error("Database failed for account model in database model layer")]
    AccountError(#[from] AccountError),

    #[error("Database failed for character model in database model layer")]
    CharacterError(#[from] CharacterError),
}
