use crate::db::models::account::error::AccountError;
use crate::db::models::char::error::CharacterError;
use crate::db::models::world::error::WorldError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ModelError {
    #[error("Database failed for account model in database model layer")]
    AccountError(#[from] AccountError),

    #[error("Database failed for character model in database model layer")]
    CharacterError(#[from] CharacterError),

    #[error("Database failed for world model in database model layer")]
    WorldError(#[from] WorldError),
}
