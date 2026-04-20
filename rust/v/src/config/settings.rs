use config::{Config, ConfigError};
use regex::Regex;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::prelude::*;

fn get_settings() -> Result<Config, ConfigError> {
    let settings = Config::builder()
        .add_source(config::Environment::default())
        .build()
        .unwrap();
    Ok(settings)
}

fn get_address() -> Result<String, ConfigError> {
    let settings = get_settings()?;
    let addr = settings.get_string("IP_ADDRESS")?;
    Ok(addr)
}

fn get_login_port() -> Result<u16, ConfigError> {
    let settings = get_settings()?;
    let port = settings.get_int("LOGIN_PORT")?;
    Ok(port as u16)
}

fn get_world_port() -> Result<u16, ConfigError> {
    let settings = get_settings()?;
    let port = settings.get_int("WORLD_PORT")?;
    Ok(port as u16)
}

pub fn get_version() -> Result<i16, ConfigError> {
    let settings = get_settings()?;
    let version = settings.get_int("VERSION")?;
    Ok(version as i16)
}

pub fn get_db_url() -> Result<String, ConfigError> {
    let settings = get_settings()?;
    let db = settings.get_string("POSTGRES_DATABASE");
    let ip = settings.get_string("IP_ADDRESS");
    let port = settings.get_int("POSTGRES_PORT");
    let user = settings.get_string("POSTGRES_USER");
    let pw = settings.get_string("POSTGRES_PASSWORD");
    let db_url = String::from("postgres://{}:{}@{}:{}/{}", &user, &pw, &ip, &port, &db);
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

pub fn get_login_server_addr() -> Result<SocketAddr, ConfigError> {
    let addr = get_address()?;
    let port = get_login_port()?;
    let octets = convert_to_ip_array(addr)?;
    let socker_addr: SocketAddr = SocketAddr::V4(SocketAddrV4::new(
        Ipv4Addr::new(octets[0], octets[1], octets[2], octets[3]),
        port,
    ));
    Ok(socker_addr)
}

pub fn get_world_server_addr() -> Result<SocketAddr, ConfigError> {
    let addr = get_address()?;
    let port = get_world_port()?;
    let octets = convert_to_ip_array(addr)?;
    let socker_addr: SocketAddr = SocketAddr::V4(SocketAddrV4::new(
        Ipv4Addr::new(octets[0], octets[1], octets[2], octets[3]),
        port,
    ));
    Ok(socker_addr)
}
