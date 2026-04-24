use crate::config::error::ConfigError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DiscordError {
    #[error("Config error in discord layer")]
    ConfigError(#[from] ConfigError),
}
