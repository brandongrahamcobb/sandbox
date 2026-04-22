use crate::constants::WorldID;
use config::{Config, ConfigError};
use regex::Regex;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

pub fn get_settings() -> Result<Config, ConfigError> {
    let settings = Config::builder()
        .add_source(config::Environment::default())
        .build()
        .unwrap();
    Ok(settings)
}

fn get_address(settings: &Config) -> Result<String, ConfigError> {
    let addr = settings.get_string("IP_ADDRESS")?;
    Ok(addr)
}

fn get_login_port(settings: &Config) -> Result<u16, ConfigError> {
    let port = settings.get_int("LOGIN_PORT")?;
    Ok(port as u16)
}

fn get_world_port(settings: &Config) -> Result<u16, ConfigError> {
    let port = settings.get_int("WORLD_PORT")?;
    Ok(port as u16)
}

pub fn get_version(settings: &Config) -> Result<i16, ConfigError> {
    let version = settings.get_int("VERSION")?;
    Ok(version as i16)
}

pub fn get_db_url(settings: &Config) -> Result<String, ConfigError> {
    let db = settings.get_string("POSTGRES_DATABASE")?;
    let ip = settings.get_string("IP_ADDRESS")?;
    let port = settings.get_int("POSTGRES_PORT")?;
    let user = settings.get_string("POSTGRES_USER")?;
    let pw = settings.get_string("POSTGRES_PASSWORD")?;
    let db_url = String::from(format!(
        "postgres://{}:{}@{}:{}/{}",
        &user, &pw, &ip, &port, &db
    ));
    Ok(db_url)
}

fn convert_to_ip_array(addr: String) -> Result<[u8; 4], ConfigError> {
    let re = Regex::new(r"^/d{3}\./d{3}\./d{3}\./d{3}").unwrap();
    let mut octets: [u8; 4] = [0u8; 4 as usize];
    for (_, [a, b, c, d]) in re.captures_iter(&addr).map(|z| z.extract()) {
        octets[0] = a.parse().unwrap();
        octets[1] = b.parse().unwrap();
        octets[2] = c.parse().unwrap();
        octets[3] = d.parse().unwrap();
    }
    Ok(octets)
}

pub fn get_login_server_addr(settings: &Config) -> Result<SocketAddr, ConfigError> {
    let addr = get_address(&settings)?;
    let port = get_login_port(&settings)?;
    let octets = convert_to_ip_array(addr)?;
    let socker_addr: SocketAddr = SocketAddr::V4(SocketAddrV4::new(
        Ipv4Addr::new(octets[0], octets[1], octets[2], octets[3]),
        port,
    ));
    Ok(socker_addr)
}

pub fn get_world_server_addr(settings: &Config) -> Result<SocketAddr, ConfigError> {
    let addr = get_address(&settings)?;
    let port = get_world_port(&settings)?;
    let octets = convert_to_ip_array(addr)?;
    let socker_addr: SocketAddr = SocketAddr::V4(SocketAddrV4::new(
        Ipv4Addr::new(octets[0], octets[1], octets[2], octets[3]),
        port,
    ));
    Ok(socker_addr)
}

pub fn get_pin_required(settings: &Config) -> Result<bool, ConfigError> {
    let pin_req = settings.get_bool("PIN_REQUIRED")?;
    Ok(pin_req)
}

pub fn get_gender_required(settings: &Config) -> Result<bool, ConfigError> {
    let gender_req = settings.get_bool("GENDER_REQUIRED")?;
    Ok(gender_req)
}

pub fn get_channel_world_pairs(settings: &Config) -> Result<Vec<(u8, u8)>, ConfigError> {
    let mut list = Vec::new();
    let mut count: i64;
    let scania: bool = settings.get_bool("SCANIA")?;
    if scania {
        count = settings.get_int("SCANIA_CHANNEL_COUNT")?;
        list.push((WorldID::SCANIA as u8, count as u8))
    }
    let bera: bool = settings.get_bool("BERA")?;
    if bera {
        count = settings.get_int("BERA_CHANNEL_COUNT")?;
        list.push((WorldID::BERA as u8, count as u8))
    }
    let windia: bool = settings.get_bool("WINDIA")?;
    if windia {
        count = settings.get_int("WINDIA_CHANNEL_COUNT")?;
        list.push((WorldID::WINDIA as u8, count as u8))
    }
    let broa: bool = settings.get_bool("BROA")?;
    if broa {
        count = settings.get_int("BROA_CHANNEL_COUNT")?;
        list.push((WorldID::BROA as u8, count as u8))
    }
    let khaini = settings.get_bool("KHAINI")?;
    if khaini {
        count = settings.get_int("KHAINI_CHANNEL_COUNT")?;
        list.push((WorldID::KHAINI as u8, count as u8))
    }
    let mardia = settings.get_bool("MARDIA")?;
    if mardia {
        count = settings.get_int("MARDIA_CHANNEL_COUNT")?;
        list.push((WorldID::MARDIA as u8, count as u8))
    }
    let yellonde = settings.get_bool("YELLONDE")?;
    if yellonde {
        count = settings.get_int("YELLONDE_CHANNEL_COUNT")?;
        list.push((WorldID::YELLONDE as u8, count as u8))
    }
    let bellocan = settings.get_bool("BELLOCAN")?;
    if bellocan {
        count = settings.get_int("BELLOCAN_CHANNEL_COUNT")?;
        list.push((WorldID::BELLOCAN as u8, count as u8))
    }
    Ok(list)
}

pub fn get_channel_capacity(settings: &Config) -> Result<u16, ConfigError> {
    let capacity = settings.get_int("CHANNEL_CAPACITY")?;
    Ok(capacity as u16)
}

pub fn get_world_flag(settings: &Config) -> Result<u8, ConfigError> {
    let flag = settings.get_int("WORLD_FLAG")?;
    Ok(flag as u8)
}

pub fn get_world_event_message(settings: &Config) -> Result<String, ConfigError> {
    let event_message = settings.get_string("EVENT_MESSAGE")?;
    Ok(event_message)
}
