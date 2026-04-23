use crate::db;
use crate::db::error::DatabaseError;
use crate::net::error::NetworkError;
use crate::net::packet::core::Packet;
use crate::net::packet::error::PacketError;
use crate::net::packet::handler::error::HandlerError;
use crate::net::packet::handler::login::action::LoginAction;
use crate::net::packet::handler::login::error::LoginError;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::io::error::IOError;
use crate::prelude::*;
use crate::runtime::error::SessionError;
use crate::runtime::relay::RuntimeContext;
use std::io::BufReader;

pub struct TOSHandler;

impl TOSHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        ctx: &RuntimeContext,
        packet: &Packet,
    ) -> Result<HandlerResult<LoginAction>, NetworkError> {
        let mut reader = BufReader::new(&**packet);
        reader
            .read_short()
            .map_err(IOError::ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let confirmed = reader
            .read_byte()
            .map_err(IOError::ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        if confirmed != 0x01 {
            return Err(NetworkError::from(PacketError::from(HandlerError::from(
                LoginError::DeniedTOS,
            ))));
        }
        let session = ctx
            .shared_state
            .sessions
            .get(ctx.session_id)
            .ok_or(SessionError::NotFound(ctx.session_id))
            .map_err(NetworkError::from)?;
        let hwid = session
            .hwid
            .ok_or(SessionError::NoHWID)
            .map_err(NetworkError::from)?;
        let account_id = session
            .account_id
            .ok_or(SessionError::NoAccount)
            .map_err(NetworkError::from)?;
        let mut acc = db::models::account::service::get_account_by_id(account_id, ctx)
            .map_err(DatabaseError::from)
            .map_err(NetworkError::from)?;
        acc.accepted_tos = true;
        db::models::account::service::update(&acc, ctx)
            .map_err(DatabaseError::from)
            .map_err(NetworkError::from)?;
        let mut result = HandlerResult::new();
        let action = LoginAction::AcceptLogin { acc, hwid };
        result.add_action(action)?;
        Ok(result)
    }
}
