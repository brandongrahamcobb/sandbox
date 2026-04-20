use crate::db::schema::accounts::dsl::*;
use crate::inc::helpers;
use crate::net::error::NetworkError;
use crate::net::packet::core::Packet;
use crate::net::packet::handler::default::PacketHandler;
use crate::net::packet::handler::error::PacketGenericError::AuthenticationError;
use crate::net::packet::handler::service::HandlerAction;
use crate::op::send::SendOpcode::Authentication;
use crate::{
    db::models::account::model::Account, net::packet::handler::service::PacketHandlerResult,
};
use bcrypt::verify;
use diesel::prelude::*;
use std::io::BufReader;
use tracing::{error, warn};

pub struct AuthenticationHandler;

impl AuthenticationHandler {
    pub fn new() -> Self {
        Self
    }

    fn authenticate(acc: Account, pw: &str) -> Result<Account, NetworkError> {
        match verify(pw, &acc.password) {
            Ok(true) => Ok(acc),
            Ok(false) => Err(AuthenticationError),
            Err(e) => {
                !error(
                    "Expected a valid account authentication. Received an unknown verification status.",
                    e.to_string(),
                );
                Err(AuthenticationError)
            }
        }
    }

    fn check_account_status(acc: &Account) -> u8 {
        if acc.banned {
            2
        } else if acc.in_overworld {
            7
        } else if !acc.accepted_tos {
            23
        } else {
            0
        }
    }

    pub fn build_successful_auth_packet(acc: &Account) -> Result<Packet, NetworkError> {
        let mut packet = Packet::new_empty();
        let opcode = Authentication as i16;
        let settings = Settings::new();
        let account_id = acc.id;
        let gender = acc.gender;
        let account_name = &acc.user_name;
        let created_at: i64 = acc.created_at.duration_since(UNIX_EPOCH).as_secs() as i64;
        packet.write_short(opcode)?;
        packet.write_int(0)?;
        packet.write_short(0)?;
        packet.write_int(account_id)?;
        packet.write_byte(gender as u8)?;
        packet.write_byte(0)?;
        packet.write_byte(0)?;
        packet.write_byte(0)?;
        packet.write_str_with_length(account_name)?;
        packet.write_byte(0)?;
        packet.write_byte(0)?;
        packet.write_long(0)?;
        packet.write_long(created_at)?;
        packet.write_int(1)?;
        packet.write_byte(settings.login.pin_required as u8)?;
        packet.write_byte(1)?;
        Ok(packet)
    }

    fn read_credentials(packet: &mut Packet) -> Result<(String, String, String), NetworkError> {
        let mut reader = BufReader::new(&**packet);
        reader.read_short()?;
        let user = reader.read_str_with_length()?;
        let pw = reader.read_str_with_length()?;
        reader.read_bytes(6)?;
        let hwid = helpers::to_hex_string(&reader.read_bytes(4)?);
        Ok((user, pw, hwid))
    }
}

impl PacketHandler for AuthenticationHandler {
    fn handle(&self, packet: &mut Packet) -> Result<PacketHandlerResult, NetworkError> {
        let (user, pw, hwid) = Self::read_credentials(packet)?;
        let acc = Self::authenticate(&user, &pw)?;
        let status = Self::check_account_status(&acc)?;
        match status {
            0 => {
                packet = build_successful_auth_packet()?;
            }
            7 => {
                packet = build_banned_auth_packet()?;
            }
            23 => {
                packet = build_tos_auth_packet()?;
            }
            _ => {
                error!(
                    "Expected authentication status code. Found an invalid status code. Status: {}",
                    status
                );
                Err(AuthenticationError)
            }
        }
        let action = HandlerAction::Reply(*packet);
        Ok(PacketHandlerResult(action))
    }
}
