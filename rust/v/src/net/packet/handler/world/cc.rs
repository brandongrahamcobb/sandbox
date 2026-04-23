use config::Config;

use crate::config::settings;
use crate::net::channel::core::Channel;
use crate::net::error::NetworkError;
use crate::net::packet::core::Packet;
use crate::net::packet::error::PacketError;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::handler::world::action::WorldAction;
use crate::net::packet::io::error::IOError::{ReadError, WriteError};
use crate::op::send::SendOpcode;
use crate::prelude::*;
use crate::runtime::error::SessionError;
use crate::runtime::relay::RuntimeContext;
use core::net::IpAddr;
use std::io::BufReader;

pub struct ChangeChannelHandler;

impl ChangeChannelHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        self: &Self,
        ctx: &RuntimeContext,
        packet: &Packet,
    ) -> Result<HandlerResult<WorldAction>, NetworkError> {
        let mut reader = BufReader::new(&**packet);
        let _op = reader
            .read_short()
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let channel_id = reader
            .read_byte()
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        // let _tick = reader
        //     .read_int()
        //     .map_err(ReadError)
        //     .map_err(PacketError::from)
        //     .map_err(NetworkError::from)?;
        let session = ctx
            .shared_state
            .sessions
            .get(ctx.session_id)
            .ok_or(SessionError::NotFound(ctx.session_id))
            .map_err(NetworkError::from)?;
        let world_id = session
            .selected_world_id
            .ok_or(SessionError::NoWorldSelected)
            .map_err(NetworkError::from)?;
        let mut result = HandlerResult::new();
        let action = WorldAction::ChangeChannel {
            channel_id,
            world_id,
        };
        result.add_action(action)?;
        Ok(result)
    }
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
