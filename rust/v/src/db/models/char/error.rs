use thiserror::Error;

#[derive(Debug, Error)]
pub enum CharacterError {
    #[error("Failed character model in database model character layer")]
    CharacterError,
}
