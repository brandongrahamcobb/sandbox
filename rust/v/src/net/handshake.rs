use crate::config::settings;
use crate::net::error::NetworkError;
use crate::net::packet::{core::Packet, write::PktWrite};

const VERSION: i16 = settings::get_version();

pub fn build_handshake_packet(
    recv_iv: &Vec<u8>,
    send_iv: &Vec<u8>,
) -> Result<Packet, NetworkError> {
    let mut packet = Packet::new_empty();
    packet.write_short(0x0E)?;
    packet.write_short(VERSION)?;
    // Not sure what this part is meant to represent...
    // HeavenClient doesn't seem to care for these values but the
    // official clients do...
    packet.write_short(0)?;
    packet.write_byte(0)?;
    packet.write_bytes(&recv_iv)?;
    packet.write_bytes(&send_iv)?;
    packet.write_byte(8)?; // Locale byte
    Ok(packet)
}
