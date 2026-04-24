use core::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

use crate::config::error::ConfigError;
use crate::helpers;
use config::Config;

pub fn get_settings() -> Result<Config, ConfigError> {
    let settings = Config::builder()
        .add_source(config::Environment::default())
        .build()
        .unwrap();
    Ok(settings)
}

pub fn get_token(settings: &Config) -> Result<String, ConfigError> {
    let key = String::from("token");
    let token = settings
        .get_string(&key)
        .map_err(|_| ConfigError::InvalidString(key))?;
    Ok(token)
}

pub fn get_intents(settings: &Config) -> Result<String, ConfigError> {
    let key = String::from("intents");
    let intents = settings
        .get_string(&key)
        .map_err(|_| ConfigError::InvalidString(key))?;
    Ok(intents)
}

pub fn get_logging_level(settings: &Config) -> Result<String, ConfigError> {
    let key = String::from("logging_level");
    let logging_level = settings
        .get_string(&key)
        .map_err(|_| ConfigError::InvalidString(key))?;
    Ok(logging_level)
}

pub fn get_core_server_addr(settings: &Config) -> Result<SocketAddr, ConfigError> {
    let addr = get_address(settings)?;
    let port: i16 = get_core_port(settings)?;
    let octets = helpers::convert_to_ip_array(addr);
    Ok(SocketAddr::V4(SocketAddrV4::new(
        Ipv4Addr::new(octets[0], octets[1], octets[2], octets[3]),
        port as u16,
    )))
}

fn get_address(settings: &Config) -> Result<String, ConfigError> {
    let key = String::from("ip_address");
    let addr = settings
        .get_string(&key)
        .map_err(|_| ConfigError::InvalidString(key))?;
    Ok(addr)
}

fn get_core_port(settings: &Config) -> Result<i16, ConfigError> {
    let key = String::from("core_port");
    let port = settings
        .get_int(&key)
        .map_err(|_| ConfigError::InvalidInt(key))?;
    Ok(port as i16)
}
