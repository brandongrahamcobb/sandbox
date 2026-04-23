use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Invalid integer value for key in config layer: {0}")]
    InvalidInt(String),

    #[error("Invalid boolean value for key in config layer: {0}")]
    InvalidBool(String),

    #[error("Invalid string value for key in config layer: {0}")]
    InvalidString(String),

    #[error("Port out of range in config layer: {0}")]
    InvalidPort(String),

    #[error("Ip address parse error in config layer: {0}")]
    InvalidIp(String),

    #[error("Integer conversion error in config layer")]
    IntConversion(#[from] std::num::TryFromIntError),

    #[error("Config backend error in config layer")]
    Source(#[from] config::ConfigError),
}
