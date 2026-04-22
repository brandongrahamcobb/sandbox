use thiserror::Error;

#[derive(Debug, Error)]
pub enum WorldError {
    #[error("Failed world model in database model world layer")]
    WorldError,
}
