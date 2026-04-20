use crate::net::packet::core::Packet;

#[derive(Debug)]
pub enum HandlerAction {
    Reply(Packet),
    // Broadcast {
    //     scope: BroadcastScope,
    //     packet: Packet,
    // },
    // Disconnect,
    // CreateSession {
    //     account_id: i32,
    //     hwid: String,
    //     state: SessionState,
    // },
    // AttachCharacter { character_id: i32 },
    // UpdateSessionSelection { world_id: u8, channel_id: u8 },
    // ReattachSession { character_id: i32, channel_id: u8 },
    // ChangeChannel { world_id: u8, channel_id: u8 },
    // Whisper {
    //     target_name: String,
    //     recipient_packet: Packet,
    //     sender_success_packet: Packet,
    //     sender_failure_packet: Packet,
    // },
    // FieldChat { packet: Packet },
    // FieldMove {
    //     packet: Packet,
    //     movement_bytes: Vec<u8>,
    // },
    // MapChanged {
    //     old_map_id: i32,
    //     new_map_id: i32,
    //     spawn_portal_id: Option<u8>,
    //     spawn_x: Option<i16>,
    //     spawn_y: Option<i16>,
    //     spawn_stance: Option<u8>,
    // },
}

#[derive(Debug, Default)]
pub struct PacketHandlerResult {
    pub actions: Vec<HandlerAction>,
}

impl PacketHandlerResult {
    pub fn empty() -> Self {
        Self { actions: vec![] }
    }

    pub fn reply(packet: Packet) -> Self {
        Self {
            actions: vec![HandlerAction::Reply(packet)],
        }
    }

    pub fn replies(packets: Vec<Packet>) -> Self {
        Self {
            actions: packets.into_iter().map(HandlerAction::Reply).collect(),
        }
    }

    // pub fn with_reply(mut self, packet: Packet) -> Self {
    //     self.actions.push(HandlerAction::Reply(packet));
    //     self
    // }
    //
    // pub fn with_broadcast(mut self, scope: BroadcastScope, packet: Packet) -> Self {
    //     self.actions
    //         .push(HandlerAction::Broadcast { scope, packet });
    //     self
    // }
    //
    // pub fn with_disconnect(mut self) -> Self {
    //     self.actions.push(HandlerAction::Disconnect);
    //     self
    // }
    //
    // pub fn with_create_session(
    //     mut self,
    //     account_id: i32,
    //     hwid: String,
    //     state: SessionState,
    // ) -> Self {
    //     self.actions.push(HandlerAction::CreateSession {
    //         account_id,
    //         hwid,
    //         state,
    //     });
    //     self
    // }
    //
    // pub fn with_attach_character(mut self, character_id: i32) -> Self {
    //     self.actions
    //         .push(HandlerAction::AttachCharacter { character_id });
    //     self
    // }
    //
    // pub fn with_update_session_selection(mut self, world_id: u8, channel_id: u8) -> Self {
    //     self.actions.push(HandlerAction::UpdateSessionSelection {
    //         world_id,
    //         channel_id,
    //     });
    //     self
    // }
    //
    // pub fn with_reattach_session(mut self, character_id: i32, channel_id: u8) -> Self {
    //     self.actions.push(HandlerAction::ReattachSession {
    //         character_id,
    //         channel_id,
    //     });
    //     self
    // }
    //
    // pub fn with_change_channel(mut self, world_id: u8, channel_id: u8) -> Self {
    //     self.actions.push(HandlerAction::ChangeChannel {
    //         world_id,
    //         channel_id,
    //     });
    //     self
    // }
    //
    // pub fn with_whisper(
    //     mut self,
    //     target_name: String,
    //     recipient_packet: Packet,
    //     sender_success_packet: Packet,
    //     sender_failure_packet: Packet,
    // ) -> Self {
    //     self.actions.push(HandlerAction::Whisper {
    //         target_name,
    //         recipient_packet,
    //         sender_success_packet,
    //         sender_failure_packet,
    //     });
    //     self
    // }
    //
    // pub fn with_field_chat(mut self, packet: Packet) -> Self {
    //     self.actions.push(HandlerAction::FieldChat { packet });
    //     self
    // }
    //
    // pub fn with_field_move(mut self, packet: Packet, movement_bytes: Vec<u8>) -> Self {
    //     self.actions.push(HandlerAction::FieldMove {
    //         packet,
    //         movement_bytes,
    //     });
    //     self
    // }
    //
    // pub fn with_map_changed(
    //     mut self,
    //     old_map_id: i32,
    //     new_map_id: i32,
    //     spawn_portal_id: Option<u8>,
    //     spawn_x: Option<i16>,
    //     spawn_y: Option<i16>,
    //     spawn_stance: Option<u8>,
    // ) -> Self {
    //     self.actions.push(HandlerAction::MapChanged {
    //         old_map_id,
    //         new_map_id,
    //         spawn_portal_id,
    //         spawn_x,
    //         spawn_y,
    //         spawn_stance,
    //     });
    //     self
    // }
}
