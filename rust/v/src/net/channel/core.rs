use crate::net::channel::error::ChannelError;
use crate::net::error::NetworkError;
use crate::net::world::error::WorldError;
use crate::runtime::state::SharedState;

#[derive(Clone, Debug)]
pub struct Channel {
    pub world_id: u8,
    pub channel_id: u8,
    pub name: String,
    pub capacity: u16,
    pub port: u16,
}

pub fn resolve_channel(
    channel_id: u8,
    world_id: u8,
    shared_state: &SharedState,
) -> Result<Channel, NetworkError> {
    for world in &shared_state.worlds {
        if world.id == world_id {
            for channel in &world.channels {
                if channel.channel_id == channel_id {
                    return Ok(channel.clone());
                }
            }
            return Err(NetworkError::from(ChannelError::NotFound(channel_id)));
        }
        return Err(NetworkError::from(WorldError::NotFound(world_id)));
    }
    Err(NetworkError::UnexpectedError)
}
