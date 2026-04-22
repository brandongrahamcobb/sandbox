use crate::net::packet::core::Packet;

#[derive(Clone)]
pub enum BroadcastScope {
    Map(i32),
    MapExcludeSelf(i32),
    World,
    WorldExcludeSelf,
    Party(i32),
    Guild(i32),
    Nearby(i32, i16, i16),
}

pub enum WorldAction {
    Broadcast {
        scope: BroadcastScope,
        packet: Packet,
    },
    Disconnect,
    ChangeChannel {
        world_id: u8,
        channel_id: u8,
    },
    Whisper {
        target_name: String,
        recipient_packet: Packet,
        sender_success_packet: Packet,
        sender_failure_packet: Packet,
    },
    FieldChat {
        packet: Packet,
    },
    FieldMove {
        packet: Packet,
        movement_bytes: Vec<u8>,
    },
    MapChanged {
        old_map_id: i32,
        new_map_id: i32,
        spawn_portal_id: Option<u8>,
        spawn_x: Option<i16>,
        spawn_y: Option<i16>,
        spawn_stance: Option<u8>,
    },
}
