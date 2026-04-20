use crate::net::error::NetworkError;
use crate::net::packet::core::Packet;
use crate::net::packet::handler::default::PacketHandler;
use crate::net::packet::handler::error::PacketGenericError::TOSError;
use crate::net::packet::handler::service::PacketHandlerResult;
use std::io::BufReader;
use tracing::error;

pub struct TOSHandler;

impl TOSHandler {
    pub fn new() -> Self {
        Self
    }
}

impl PacketHandler for TOSHandler {
    fn handle(&self, packet: &mut Packet) -> Result<PacketHandlerResult, NetworkError> {
        let mut reader = BufReader::new(&**packet);
        reader.read_short()?;
        let confirmed = reader.read_byte()?;
        if confirmed != 0x01 {
            packet = build_tos_deny_packet()?;
            // self.writer.send_packet(&mut packet).await?
        }
        // NOT IMPLEMENTED
    }
}
// let account_id = ctx
//     .session
//     .session
//     .as_ref()
//     .map(|s| s.account_id)
//     .ok_or(NetworkError::NotLoggedIn)?;
// let mut user = account::get_account_by_id(account_id)?;
// user.accepted_tos = true;
// let user = account::update_account(&user)?;
// let login_packet = build::login::status::build_successful_login_packet(&user)?;
// Ok(HandlerResult::reply(login_packet))
