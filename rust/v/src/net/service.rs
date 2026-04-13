use crate::helpers::to_hex_string;
use crate::io::packet::Packet;
use crate::io::read::PktRead;
use crate::net::error::NetworkError;
use crate::net::login;
use std::io::BufReader;
pub struct LoginCredentialsHandler;

impl LoginCredentialsHandler {
    pub fn new() -> Self {
        Self
    }

    fn read_credentials(packet: &mut Packet) -> Result<(String, String, String), NetworkError> {
        let mut reader = BufReader::new(packet.bytes.as_slice());
        reader.read_short()?;
        let user = reader.read_str_with_length()?;
        let pw = reader.read_str_with_length()?;
        reader.read_bytes(6)?;
        let hwid = to_hex_string(&reader.read_bytes(4)?);
        Ok((user, pw, hwid))
    }

    pub fn handle(packet: &mut Packet) -> Result<Packet, NetworkError> {
        let (_user, _pw, _hwid) = Self::read_credentials(packet)?;
        login::build_login_status_packet(0)
    }
}
