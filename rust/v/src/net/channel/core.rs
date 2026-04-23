use crate::{
    net::{channel::error::ChannelError, error::NetworkError, world::error::WorldError},
    runtime::state::SharedState,
};

#[derive(Clone, Debug)]
pub struct Channel {
    pub world_id: u8,
    pub channel_id: u8,
    pub name: String,
    pub capacity: u16,
}

pub fn resolve_channel(
    channel_id: u8,
    world_id: u8,
    state: &SharedState,
) -> Result<Channel, NetworkError> {
    for world in state.worlds {
        if world.id == world_id {
            for channel in world.channels {
                if channel.channel_id == channel.id {
                    return channel;
                }
            }
            Err(NetworkError::from(ChannelError::NotFound))
        }
        Err(NetworkError::from(WorldError::NotFound))
    }
    Err(NetworkError::UnexpectedError)
}
