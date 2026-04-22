use crate::config::settings;
use crate::constants::WORLDS;
use crate::net::channel::core::Channel;
use crate::net::error::NetworkError;
use crate::runtime::state::SharedState;

pub struct WorldInfo {
    pub id: u8,
    pub name: &'static str,
}

#[derive(Clone, Debug)]
pub struct World {
    pub id: u8,
    pub name: String,
    pub flag: u8,
    pub event_message: String,
    pub channels: Vec<Channel>,
}

pub fn load_worlds(state: &SharedState) -> Result<Vec<World>, NetworkError> {
    let mut worlds: Vec<World> = Vec::new();
    let capacity: u16 = settings::get_channel_capacity(&state.settings)?;
    let flag: u8 = settings::get_world_flag(&state.settings)?;
    let event_message: String = settings::get_world_event_message(&state.settings)?;
    let pairs: Vec<(u8, u8)> = settings::get_channel_world_pairs(&state.settings)?;
    for (id, count) in pairs {
        let name: &str = name_for_world_by_id(id).unwrap_or("Unkown");
        let channels: Vec<Channel> = (0..count)
            .map(|channel_id| Channel {
                world_id: id,
                channel_id,
                name: format!("{name}-{}", channel_id + 1),
                capacity: capacity,
            })
            .collect();
        worlds.push(World {
            id,
            name: name.to_string(),
            flag,
            event_message: event_message.clone(),
            channels,
        })
    }
    Ok(worlds)
}

pub fn name_for_world_by_id(id: u8) -> Option<&'static str> {
    WORLDS.get(id as usize).map(|w| w.name)
}
