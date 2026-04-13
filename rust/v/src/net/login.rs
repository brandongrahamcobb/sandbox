use crate::io::{packet::Packet, write::PktWrite};
use crate::net::error::NetworkError;
use crate::op::send::SendOpcode::LoginStatus;

pub fn build_login_status_packet(status: u8) -> Result<Packet, NetworkError> {
    // TODO: Need to create an enum for the status...
    let mut packet = Packet::new_empty();
    let opcode = LoginStatus as i16;
    packet.write_short(opcode)?;
    packet.write_byte(status)?;
    packet.write_byte(0)?;
    packet.write_int(0)?;
    Ok(packet)
}

pub fn build_handshake_packet(
    recv_iv: &Vec<u8>,
    send_iv: &Vec<u8>,
) -> Result<Packet, NetworkError> {
    let mut packet = Packet::new_empty();
    packet.write_short(0x0E)?;
    packet.write_short(83)?; // Version
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
