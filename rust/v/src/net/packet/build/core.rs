use crate::config::settings;
use crate::db::models::account::core::Account;
use crate::net::channel::core::Channel;
use crate::net::error::NetworkError;
use crate::net::packet::core::Packet;
use crate::net::packet::error::PacketError;
use crate::net::packet::io::error::IOError::WriteError;
use crate::net::packet::io::write::PktWrite;
use crate::op::send::SendOpcode;
use crate::runtime::relay::RuntimeContext;
use crate::runtime::state::SharedState;
use config::Config;
use std::net::IpAddr;
use std::time::UNIX_EPOCH;

pub fn build_handshake_packet(
    recv_iv: &Vec<u8>,
    send_iv: &Vec<u8>,
    shared_state: &SharedState,
) -> Result<Packet, NetworkError> {
    let mut packet = Packet::new_empty();
    let version = settings::get_version(&shared_state.settings)?;
    packet
        .write_short(0x0E)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_short(version)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;

    // Not sure what this part is meant to represent...
    // HeavenClient doesn't seem to care for these values but the
    // official clients do...
    packet
        .write_short(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_byte(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_bytes(&recv_iv)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_bytes(&send_iv)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_byte(8) // Locale byte
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    Ok(packet)
}

pub fn build_failed_login_packet(status: u8) -> Result<Packet, NetworkError> {
    let mut packet = Packet::new_empty();
    let opcode = SendOpcode::AccountStatus as u16;
    packet
        .write_short(opcode)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_byte(status)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_byte(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_int(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    Ok(packet)
}

pub fn build_successful_login_packet(
    acc: &Account,
    ctx: &RuntimeContext,
) -> Result<Packet, NetworkError> {
    let mut packet = Packet::new_empty();
    let opcode = SendOpcode::AccountStatus as u16;
    let account_id = acc.id as u32;
    let gender = acc.gender;
    let account_name = &acc.username;
    let created_at: u64 = acc.created_at.duration_since(UNIX_EPOCH)?.as_secs();
    let pin_required = settings::get_pin_required(&ctx.shared_state.settings)?;
    packet
        .write_short(opcode)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_int(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_short(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_int(account_id)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_byte(gender as u8)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_byte(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_byte(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_byte(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_str_with_length(account_name)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_byte(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_byte(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_long(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_long(created_at)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_int(1)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_byte(pin_required as u8)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_byte(1)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    Ok(packet)
}

pub fn build_channel_change_packet(
    channel: &Channel,
    settings: &Config,
) -> Result<Packet, NetworkError> {
    let mut packet = Packet::new_empty();
    let addr = settings::get_world_server_addr(&settings)?;
    let v4 = match addr.ip() {
        IpAddr::V4(v4) => v4,
        IpAddr::V6(_) => return Err(NetworkError::UnexpectedError),
    };
    let op = SendOpcode::ChangeChannel as u16;
    packet
        .write_short(op)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_byte(1)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_bytes(&v4.octets())
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_short(channel.port)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    Ok(packet)
}
