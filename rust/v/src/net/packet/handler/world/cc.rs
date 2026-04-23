use crate::net::error::NetworkError;
use crate::net::packet::core::Packet;
use crate::net::packet::error::PacketError;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::handler::world::action::WorldAction;
use crate::net::packet::io::error::IOError;
use crate::prelude::*;
use crate::runtime::error::SessionError;
use crate::runtime::relay::RuntimeContext;
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
            .map_err(IOError::ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let channel_id = reader
            .read_byte()
            .map_err(IOError::ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        // let _tick = reader
        //     .read_int()
        //     .map_err(IOError::ReadError)
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
        result.add_action(action);
        Ok(result)
    }
}
