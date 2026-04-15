use config::{Config, ConfigError};
use regex::Regex;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpStream};

fn get_address(settings: &Config) -> Result<String, ConfigError> {
    let addr = settings.get_string("SERVER_ADDR")?;
    Ok(addr)
}

fn get_port(settings: &Config) -> Result<u16, ConfigError> {
    let port = settings.get_int("LOGIN_PORT")?;
    Ok(port as u16)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tcp() {
        let settings = Config::builder()
            .add_source(config::File::with_name("config.yaml"))
            .build()
            .unwrap();
        let addr = get_address(&settings).unwrap();
        let login_port = get_port(&settings).unwrap();
        let re = Regex::new(r"^/d{3}\./d{3}\./d{3}\./d{3}").unwrap();
        let mut parameters: [u8; 4] = [0; 4];
        for (_, [a, b, c, d]) in re.captures_iter(&addr).map(|z| z.extract()) {
            parameters[0] = a.parse().unwrap();
            parameters[1] = b.parse().unwrap();
            parameters[2] = c.parse().unwrap();
            parameters[3] = d.parse().unwrap();
        }
        let sock_ipv4_addr: SocketAddr = SocketAddr::V4(SocketAddrV4::new(
            Ipv4Addr::new(parameters[0], parameters[1], parameters[2], parameters[3]),
            login_port,
        ));
        let stream = TcpStream::connect(sock_ipv4_addr).expect("Couldn't connect to the server...");
        assert_eq!(
            stream.peer_addr().unwrap(),
            SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8484))
        );
    }
}
