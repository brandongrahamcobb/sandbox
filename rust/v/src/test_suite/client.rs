#[cfg(test)]
mod tests {
    use crate::io::write::PktWrite;
    use crate::op::send::SendOpcode::LoginStatus;
    use crate::sec::aes::AES;
    use crate::sec::custom;
    use crate::test_suite::settings;
    use rand::{RngExt, random, rng};
    use regex::Regex;
    use std::io::prelude::*;
    use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpStream};

    fn convert_to_ip_array(addr: &String) -> Result<[u8; 4], std::io::Error> {
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

    fn connect(addr: &String, port: u16) -> Result<TcpStream, std::io::Error> {
        let octets = convert_to_ip_array(&addr).unwrap();
        let sock_ipv4_addr: SocketAddr = SocketAddr::V4(SocketAddrV4::new(
            Ipv4Addr::new(octets[0], octets[1], octets[2], octets[3]),
            port,
        ));
        let stream = TcpStream::connect(sock_ipv4_addr)?;
        Ok(stream)
    }

    #[test]
    fn test_login() -> Result<(), std::io::Error> {
        let env = settings::get_settings().unwrap();
        let addr = settings::get_address(&env).unwrap();
        let login_port = settings::get_login_port(&env).unwrap();
        let stream = connect(&addr, login_port)?;
        assert_eq!(
            stream.peer_addr().unwrap(),
            SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8484))
        );
        Ok(())
    }

    #[test]
    fn test_world() -> Result<(), std::io::Error> {
        let env = settings::get_settings().unwrap();
        let addr = settings::get_address(&env).unwrap();
        let world_port = settings::get_world_port(&env).unwrap();
        let stream = connect(&addr, world_port)?;
        assert_eq!(
            stream.peer_addr().unwrap(),
            SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8485))
        );
        Ok(())
    }
}
