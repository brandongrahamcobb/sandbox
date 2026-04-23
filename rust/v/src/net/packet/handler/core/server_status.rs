use crate::net::error::NetworkError;
use crate::net::packet::core::Packet;
use crate::net::packet::error::PacketError;
use crate::net::packet::handler::core::action::CoreAction;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::io::error::IOError::WriteError;
use crate::net::world;
use crate::op::send::SendOpcode;
use crate::prelude::*;
use crate::runtime::relay::RuntimeContext;

pub struct ServerStatusHandler;

impl ServerStatusHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        self: &Self,
        ctx: &RuntimeContext,
        _packet: &Packet,
    ) -> Result<HandlerResult<CoreAction>, NetworkError> {
        let worlds = world::core::load_worlds(&ctx.shared_state.settings)?;
        let status: u16 = if worlds.iter().any(|world| !world.channels.is_empty()) {
            0
        } else {
            2
        };
        let packet = build_server_status_packet(status)?;
        let action = CoreAction::Simple { packet };
        let mut result = HandlerResult::new();
        result.add_action(action)?;
        Ok(result)
    }
}
pub fn build_server_status_packet(status: u16) -> Result<Packet, NetworkError> {
    let mut packet = Packet::new_empty();
    let op = SendOpcode::ServerStatus as u16;
    packet
        .write_short(op)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_short(status) // Highly populated status!
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    Ok(packet)
}
