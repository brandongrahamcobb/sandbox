use thiserror::Error;

#[derive(Debug, Error)]
pub enum AccountError {
    #[error("Failed account model in database model account layer")]
    AccountError,
}
