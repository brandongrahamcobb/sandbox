use crate::config::settings;
use crate::constants::WORLDS;
use crate::net::error::NetworkError;
use crate::net::packet::core::Packet;
use crate::net::packet::error::PacketError;
use crate::net::packet::handler::core::action::CoreAction;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::io::error::IOError::WriteError;
use crate::net::world;
use crate::net::world::core::World;
use crate::op::send::SendOpcode;
use crate::prelude::*;
use crate::runtime::relay::RuntimeContext;

pub struct WorldListHandler;

impl WorldListHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        self: &Self,
        ctx: &RuntimeContext,
        _packet: &Packet,
    ) -> Result<HandlerResult<CoreAction>, NetworkError> {
        let mut result = HandlerResult::new();
        let packets = build_world_packets(&ctx)?;
        for packet in packets {
            let action = CoreAction::Simple { packet };
            result.add_action(action);
        }
        Ok(result)
    }
}

fn build_world_packets(ctx: &RuntimeContext) -> Result<Vec<Packet>, NetworkError> {
    let worlds = world::core::load_worlds(&ctx.shared_state.settings)?;
    let mut packets: Vec<Packet> = worlds
        .iter()
        .map(build_world_details_packet)
        .collect::<Result<Vec<_>, _>>()?;
    packets.push(build_world_list_packet()?);
    packets.push(build_last_connected_world_packet()?);
    packets.push(build_recommended_worlds_packet(&ctx)?);
    Ok(packets)
}

fn build_world_list_packet() -> Result<Packet, NetworkError> {
    let mut packet = Packet::new_empty();
    let op = SendOpcode::ServerList as u16;
    packet
        .write_short(op)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_byte(0xFF)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    Ok(packet)
}

fn build_last_connected_world_packet() -> Result<Packet, NetworkError> {
    let mut packet = Packet::new_empty();
    let op = SendOpcode::LastConnectedWorld as u16;
    packet
        .write_short(op)
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

fn build_recommended_worlds_packet(ctx: &RuntimeContext) -> Result<Packet, NetworkError> {
    let mut packet = Packet::new_empty();
    let op = SendOpcode::RecommendedWorlds as u16;
    packet
        .write_short(op)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    let recommended_world_names = settings::get_recommended_worlds(&ctx.shared_state.settings)?;
    let count: u8 = recommended_world_names.len().try_into().unwrap();
    if count != 0 {
        packet
            .write_byte(0)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        packet
            .write_byte(count)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        for world in WORLDS {
            for world_name in &recommended_world_names {
                if world_name == &world.name {
                    let id = world.id;
                    packet
                        .write_int(id as u32)
                        .map_err(WriteError)
                        .map_err(PacketError::from)
                        .map_err(NetworkError::from)?;
                    packet
                        .write_str(world.name)
                        .map_err(WriteError)
                        .map_err(PacketError::from)
                        .map_err(NetworkError::from)?;
                    packet
                        .write_int(0)
                        .map_err(WriteError)
                        .map_err(PacketError::from)
                        .map_err(NetworkError::from)?;
                }
            }
        }
    } else {
        packet
            .write_byte(1)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        packet
            .write_int(0)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        packet
            .write_int(0)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
    }
    Ok(packet)
}

fn build_world_details_packet(world: &World) -> Result<Packet, NetworkError> {
    let mut packet = Packet::new_empty();
    let op = SendOpcode::ServerList as u16;
    packet
        .write_short(op)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_byte(world.id)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_str_with_length(world.name.as_str())
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_byte(world.flag)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_str_with_length(world.event_message.as_str())
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_byte(100)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_byte(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_byte(100)
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
        .write_byte(world.channels.len() as u8)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    for channel in &world.channels {
        packet
            .write_str_with_length(channel.name.as_str())
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        packet
            .write_int(channel.capacity as u32)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        packet
            .write_byte(1)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        packet
            .write_byte(channel.channel_id)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        packet
            .write_byte(world.id)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
    }
    packet
        .write_short(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    Ok(packet)
}
