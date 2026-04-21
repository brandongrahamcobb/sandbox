use crate::db::schema::accounts::dsl::*;
use crate::inc::helpers;
use crate::net::error::NetworkError;
use crate::net::packet::core::Packet;
use crate::net::packet::handler::login::error::AuthenticationError;
use crate::net::packet::handler::service::HandlerAction;
use crate::net::packet::handler::service::PacketHandler;
use crate::op::recv::RecvOpcode;
use crate::op::send::SendOpcode;
use crate::{
    db::models::account::model::Account, net::packet::handler::service::PacketHandlerResult,
};
use bcrypt::verify;
use std::io::BufReader;
use tracing::{error, warn};

pub enum StatusCode {
    Banned = 2,
    PendingTOS = 7,
    Playing = 23,
}

pub struct CredentialsHandler;

impl CredentialsHandler {
    pub fn new() -> Self {
        Self
    }

    fn authenticate(acc: Account, pw: &str) -> Result<Account, NetworkError> {
        match verify(pw, &acc.password) {
            Ok(true) => Ok(acc),
            Ok(false) => Err(AuthenticationError::InvalidCredentials),
            _ => Err(AuthenticationError::UnhandledAuthenticationError),
        }
    }

    fn check_if_banned(acc: &Account) -> Result<(), NetworkError> {
        if !acc.banned {
            Ok(())
        }
        Err(AuthenticationError::Banned)
    }

    fn check_if_pending_tos(acc: &Account) -> Result<(), NetworkError> {
        if acc.accepted_tos {
            Ok(())
        }
        Err(AuthenticationError::PendingTOS)
    }

    fn check_if_playing(acc: &Account) -> Result<(), NetworkError> {
        if acc.playing {
            Ok(())
        }
        Err(AuthenticationError::Playing)
    }

    fn check_status(acc: &Account) -> Result<(), NetworkError> {
        check_if_banned(&acc)?;
        check_if_pending_tos(&acc)?;
        check_if_playing(&acc)?;
        Ok(())
    }

    pub fn build_failed_packet(acc: &Account, status: i16) -> Result<Packet, NetworkError> {
        let mut packet = Packet::new_empty();
        let opcode = SendOpcode::AccountStatus as i16;
        packet.write_short(opcode)?;
        packet.write_byte(status)?;
        packet.write_byte(0)?;
        packet.write_int(0)?;
        Ok(packet)
    }

    pub fn build_success_packet(acc: &Account) -> Result<Packet, NetworkError> {
        let mut packet = Packet::new_empty();
        let opcode = SendOpcode::AccountStatus as i16;
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

    fn read_credentials(packet: &Packet) -> Result<(String, String, String), NetworkError> {
        let mut reader = BufReader::new(&**packet);
        reader.read_short()?;
        let user = reader.read_str_with_length()?;
        let pw = reader.read_str_with_length()?;
        reader.read_bytes(6)?;
        let hwid = helpers::to_hex_string(&reader.read_bytes(4)?);
        Ok((user, pw, hwid))
    }

    pub fn handle(&self, packet: &Packet) -> Result<PacketHandlerResult, NetworkError> {
        let (user, pw, hwid) = Self::read_credentials(&packet)?;
        let acc = Self::authenticate(&user, &pw)?;
        match Self::check_status(&acc) {
            Ok(()) => {
                packet = build_success_packet(&acc)?;
            }
            Err(AuthenticationError::Banned) => {
                packet = build_failed_packet(&acc, &StatusCode::Banned)?;
            }
            Err(AuthenticationError::PendingTOS) => {
                packet = build_failed_packet(&acc, &StatusCode::PendingTOS)?;
            }
            Err(AuthenticationError::Playing) => {
                packet = build_failed_packet(&acc, &StatusCode::Playing)?;
            }
            _ => Err(AuthenticationError::UnhandledAuthenticationError),
        }
        let action = HandlerAction::Reply(*packet);
        Ok(PacketHandlerResult(action))
    }
}
