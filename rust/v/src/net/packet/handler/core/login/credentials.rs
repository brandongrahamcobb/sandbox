use crate::config::settings;
use crate::db;
use crate::db::error::DatabaseError;
use crate::db::models::account::core::Account;
use crate::inc::helpers;
use crate::net::error::NetworkError;
use crate::net::packet::core::Packet;
use crate::net::packet::error::PacketError;
use crate::net::packet::handler::error::HandlerError;
use crate::net::packet::handler::core::action::{CoreAction, RejectLoginReason};
use crate::net::packet::handler::core::login::error::LoginError;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::io::error::IOError::{ReadError, WriteError};
use crate::op::send::SendOpcode;
use crate::prelude::*;
use crate::runtime::relay::RuntimeContext;
use bcrypt::{DEFAULT_COST, hash, verify};
use std::io::BufReader;
use std::time::UNIX_EPOCH;

pub enum StatusCode {
    Banned = 2,
    InvalidCredentials = 5,
    PendingTOS = 7,
    Playing = 23,
}

pub struct CredentialsHandler;

impl CredentialsHandler {
    pub fn new() -> Self {
        Self
    }

    fn authenticate(self: &Self, acc: &Account, pw: &str) -> Result<bool, NetworkError> {
        let hash = hash(&acc.password, DEFAULT_COST)?;
        match verify(pw, &hash) {
            Ok(true) => Ok(true),
            Ok(false) => Err(NetworkError::from(PacketError::from(HandlerError::from(
                LoginError::InvalidCredentials,
            )))),
            Err(e) => Err(NetworkError::from(PacketError::from(HandlerError::from(
                LoginError::UnexpectedError,
            )))),
        }
    }

    fn check_if_banned(self: &Self, acc: &Account) -> Result<bool, NetworkError> {
        if acc.banned {
            return Ok(true);
        }
        return Ok(false);
    }

    fn check_if_pending_tos(self: &Self, acc: &Account) -> Result<bool, NetworkError> {
        if !acc.accepted_tos {
            return Ok(true);
        }
        return Ok(false);
    }

    fn check_if_playing(self: &Self, acc: &Account) -> Result<bool, NetworkError> {
        if acc.playing {
            return Ok(true);
        }
        return Ok(false);
    }

    fn get_login_action(&self, acc: Account, hwid: String) -> Result<CoreAction, NetworkError> {
        if self.check_if_banned(&acc)? {
            return Ok(CoreAction::RejectLogin {
                reason: RejectLoginReason::Banned,
                acc: Some(acc),
            });
        }
        if self.check_if_pending_tos(&acc)? {
            return Ok(CoreAction::RejectLogin {
                reason: RejectLoginReason::PendingTOS,
                acc: Some(acc),
            });
        }
        if self.check_if_playing(&acc)? {
            return Ok(CoreAction::RejectLogin {
                reason: RejectLoginReason::Playing,
                acc: Some(acc),
            });
        }
        Ok(CoreAction::AcceptLogin { acc, hwid })
    }

    fn read_credentials(
        self: &Self,
        packet: &Packet,
    ) -> Result<(String, String, String), NetworkError> {
        let mut reader = BufReader::new(&**packet);
        reader
            .read_short()
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let user = reader
            .read_str_with_length()
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let pw = reader
            .read_str_with_length()
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        reader
            .read_bytes(6)
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let hwid = helpers::to_hex_string(
            &reader
                .read_bytes(4)
                .map_err(ReadError)
                .map_err(PacketError::from)
                .map_err(NetworkError::from)?,
        );
        Ok((user, pw, hwid))
    }

    pub async fn handle(
        self: &Self,
        ctx: &RuntimeContext,
        packet: &Packet,
    ) -> Result<HandlerResult<CoreAction>, NetworkError> {
        let mut result = HandlerResult::new();
        let (user, pw, hwid) = self.read_credentials(packet)?;
        match db::models::account::service::get_account_by_username(&user, ctx) {
            Err(e) if e == diesel::result::Error::NotFound => {
                let login_action = CoreAction::RejectLogin {
                    acc: None,
                    reason: RejectLoginReason::InvalidCredentials,
                };
                result.add_action(login_action)?;
                Ok(result)
            }
            Err(e) => Err(NetworkError::from(DatabaseError::from(e))),
            Ok(acc) => {
                let login_action = {
                    if self.authenticate(&acc, &pw)? {
                        self.get_login_action(acc, hwid)?
                    } else {
                        CoreAction::RejectLogin {
                            acc: None,
                            reason: RejectLoginReason::InvalidCredentials,
                        }
                    }
                };
                result.add_action(login_action)?;
                Ok(result)
            }
        }
    }
}

pub fn build_failed_login_packet(status: u8) -> Result<Packet, NetworkError> {
    let mut packet = Packet::new_empty();
    let opcode = SendOpcode::AccountStatus as u16;
    packet
        .write_short(opcode)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_byte(status)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_byte(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_int(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    Ok(packet)
}

pub fn build_successful_login_packet(
    acc: &Account,
    ctx: &RuntimeContext,
) -> Result<Packet, NetworkError> {
    let mut packet = Packet::new_empty();
    let opcode = SendOpcode::AccountStatus as u16;
    let account_id = acc.id as u32;
    let gender = acc.gender;
    let account_name = &acc.username;
    let created_at: u64 = acc.created_at.duration_since(UNIX_EPOCH)?.as_secs();
    let pin_required = settings::get_pin_required(&ctx.shared_state.settings)?;
    packet
        .write_short(opcode)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_int(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_short(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_int(account_id)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_byte(gender as u8)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_byte(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_byte(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_byte(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_str_with_length(account_name)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_byte(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_byte(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_long(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_long(created_at)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_int(1)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_byte(pin_required as u8)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_byte(1)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    Ok(packet)
}
