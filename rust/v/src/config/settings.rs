use crate::config::error::ConfigError;
use crate::constants::WorldID;
use crate::inc::helpers;
use config::Config;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

pub fn get_settings() -> Result<Config, ConfigError> {
    let settings = Config::builder()
        .add_source(config::Environment::default())
        .build()
        .unwrap();
    Ok(settings)
}

fn get_address(settings: &Config) -> Result<String, ConfigError> {
    let key = String::from("ip_address");
    let addr = settings
        .get_string(&key)
        .map_err(|_| ConfigError::InvalidString(key))?;
    Ok(addr)
}

fn get_login_port(settings: &Config) -> Result<u16, ConfigError> {
    let key = String::from("login_port");
    let port = settings
        .get_int(&key)
        .map_err(|_| ConfigError::InvalidInt(key))?;
    Ok(port as u16)
}

pub fn get_world_port(settings: &Config) -> Result<u16, ConfigError> {
    let key = String::from("world_port");
    let port = settings
        .get_int(&key)
        .map_err(|_| ConfigError::InvalidInt(key))?;
    Ok(port as u16)
}

pub fn get_version(settings: &Config) -> Result<u16, ConfigError> {
    let key = String::from("version");
    let version = settings
        .get_int(&key)
        .map_err(|_| ConfigError::InvalidInt(key))?;
    Ok(version as u16)
}

pub fn get_db_url(settings: &Config) -> Result<String, ConfigError> {
    let db_key = String::from("postgres_database");
    let ip_key = String::from("postgres_address");
    let port_key = String::from("postgres_port");
    let user_key = String::from("postgres_user");
    let pw_key = String::from("postgres_password");
    let db = settings
        .get_string(&db_key)
        .map_err(|_| ConfigError::InvalidString(db_key))?;
    let ip = settings
        .get_string(&ip_key)
        .map_err(|_| ConfigError::InvalidString(ip_key))?;
    let port = settings
        .get_int(&port_key)
        .map_err(|_| ConfigError::InvalidInt(port_key))?;
    let user = settings
        .get_string(&user_key)
        .map_err(|_| ConfigError::InvalidString(user_key))?;
    let pw = settings
        .get_string(&pw_key)
        .map_err(|_| ConfigError::InvalidString(pw_key))?;
    Ok(format!("postgres://{}:{}@{}:{}/{}", user, pw, ip, port, db))
}

pub fn get_login_server_addr(settings: &Config) -> Result<SocketAddr, ConfigError> {
    let addr = get_address(settings)?;
    let port: u16 = get_login_port(settings)?;
    let octets = helpers::convert_to_ip_array(addr);
    Ok(SocketAddr::V4(SocketAddrV4::new(
        Ipv4Addr::new(octets[0], octets[1], octets[2], octets[3]),
        port,
    )))
}

pub fn get_world_server_addr(settings: &Config) -> Result<SocketAddr, ConfigError> {
    let addr = get_address(settings)?;
    let port: u16 = get_world_port(settings)?;
    let octets = helpers::convert_to_ip_array(addr);
    Ok(SocketAddr::V4(SocketAddrV4::new(
        Ipv4Addr::new(octets[0], octets[1], octets[2], octets[3]),
        port,
    )))
}

pub fn get_pin_required(settings: &Config) -> Result<bool, ConfigError> {
    let key = String::from("pin_required");
    let pin_req = settings
        .get_bool(&key)
        .map_err(|_| ConfigError::InvalidBool(key))?;
    Ok(pin_req)
}

pub fn get_gender_required(settings: &Config) -> Result<bool, ConfigError> {
    let key = String::from("gender_required");
    let gender_req = settings
        .get_bool(&key)
        .map_err(|_| ConfigError::InvalidBool(key))?;
    Ok(gender_req)
}

pub fn get_channel_world_pairs(settings: &Config) -> Result<Vec<(u8, u8)>, ConfigError> {
    let mut list = Vec::new();
    let worlds = [
        ("scania", WorldID::SCANIA),
        ("bera", WorldID::BERA),
        ("windia", WorldID::WINDIA),
        ("broa", WorldID::BROA),
        ("khaini", WorldID::KHAINI),
        ("mardia", WorldID::MARDIA),
        ("yellonde", WorldID::YELLONDE),
        ("bellocan", WorldID::BELLOCAN),
    ];
    for (name, id) in worlds {
        if settings.get_bool(name)? {
            let key = String::from(format!("{name}_channel_count"));
            let count: u8 = settings
                .get_int(&key)?
                .try_into()
                .map_err(|e| ConfigError::IntConversion(e))?;
            list.push((id as u8, count));
        }
    }
    Ok(list)
}

pub fn get_channel_capacity(settings: &Config) -> Result<u16, ConfigError> {
    let key = String::from("channel_capacity");
    let capacity = settings
        .get_int(&key)
        .map_err(|_| ConfigError::InvalidInt(key))?;
    Ok(capacity as u16)
}

pub fn get_world_flag(settings: &Config) -> Result<u8, ConfigError> {
    let key = String::from("world_flag");
    let flag = settings
        .get_int(&key)
        .map_err(|_| ConfigError::InvalidInt(key))?;
    Ok(flag as u8)
}

pub fn get_world_event_message(settings: &Config) -> Result<String, ConfigError> {
    let key = String::from("event_message");
    let msg = settings
        .get_string(&key)
        .map_err(|_| ConfigError::InvalidString(key))?;
    Ok(msg)
}

pub fn get_recommended_worlds(settings: &Config) -> Result<Vec<String>, ConfigError> {
    let key = String::from("recommended_worlds");
    let worlds: Vec<String> = settings
        .get_string(&key)
        .map_err(|_| ConfigError::InvalidString(key))?
        .split(",")
        .map(|s| s.trim().to_string())
        .collect::<Vec<String>>();
    Ok(worlds)
}
