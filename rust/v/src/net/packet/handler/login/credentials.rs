use crate::db;
use crate::db::error::DatabaseError;
use crate::db::models::account::core::Account;
use crate::inc::helpers;
use crate::net::error::NetworkError;
use crate::net::packet::core::Packet;
use crate::net::packet::error::PacketError;
use crate::net::packet::handler::error::HandlerError;
use crate::net::packet::handler::login::action::{LoginAction, RejectReason};
use crate::net::packet::handler::login::error::LoginError;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::io::error::IOError;
use crate::prelude::*;
use crate::runtime::relay::RuntimeContext;
use bcrypt::verify;
use std::io::BufReader;

pub enum StatusCode {
    InvalidCredentials = 0,
    Banned = 2,
    PendingTOS = 7,
    Playing = 23,
}

pub struct CredentialsHandler;

impl CredentialsHandler {
    pub fn new() -> Self {
        Self
    }

    fn authenticate(self: &Self, acc: &Account, pw: &str) -> Result<bool, NetworkError> {
        match verify(pw, &acc.password) {
            Ok(true) => Ok(true),
            Ok(false) => Err(NetworkError::from(PacketError::from(HandlerError::from(
                LoginError::InvalidCredentials,
            )))),
            _ => Err(NetworkError::from(PacketError::from(HandlerError::from(
                LoginError::UnexpectedError,
            )))),
        }
    }

    fn check_if_banned(self: &Self, acc: &Account) -> Result<bool, NetworkError> {
        if !acc.banned {
            return Ok(true);
        }
        Err(NetworkError::from(PacketError::from(HandlerError::from(
            LoginError::Banned,
        ))))
    }

    fn check_if_pending_tos(self: &Self, acc: &Account) -> Result<bool, NetworkError> {
        if acc.accepted_tos {
            return Ok(true);
        }
        Err(NetworkError::from(PacketError::from(HandlerError::from(
            LoginError::PendingTOS,
        ))))
    }

    fn check_if_playing(self: &Self, acc: &Account) -> Result<bool, NetworkError> {
        if acc.playing {
            return Ok(true);
        }
        Err(NetworkError::from(PacketError::from(HandlerError::from(
            LoginError::Playing,
        ))))
    }

    fn get_login_action(&self, acc: Account, hwid: String) -> Result<LoginAction, NetworkError> {
        if self.check_if_banned(&acc)? {
            return Ok(LoginAction::RejectLogin {
                reason: RejectReason::Banned,
                acc: Some(acc),
            });
        }
        if self.check_if_pending_tos(&acc)? {
            return Ok(LoginAction::RejectLogin {
                reason: RejectReason::PendingTOS,
                acc: Some(acc),
            });
        }
        if self.check_if_playing(&acc)? {
            return Ok(LoginAction::RejectLogin {
                reason: RejectReason::Playing,
                acc: Some(acc),
            });
        }
        Ok(LoginAction::AcceptLogin { acc, hwid })
    }

    fn read_credentials(
        self: &Self,
        packet: &Packet,
    ) -> Result<(String, String, String), NetworkError> {
        let mut reader = BufReader::new(&**packet);
        reader
            .read_short()
            .map_err(IOError::ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let user = reader
            .read_str_with_length()
            .map_err(IOError::ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let pw = reader
            .read_str_with_length()
            .map_err(IOError::ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        reader
            .read_bytes(6)
            .map_err(IOError::ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let hwid = helpers::to_hex_string(
            &reader
                .read_bytes(4)
                .map_err(IOError::ReadError)
                .map_err(PacketError::from)
                .map_err(NetworkError::from)?,
        );
        Ok((user, pw, hwid))
    }

    pub async fn handle(
        self: &Self,
        ctx: &RuntimeContext,
        packet: &Packet,
    ) -> Result<HandlerResult<LoginAction>, NetworkError> {
        let mut result = HandlerResult::new();
        let (user, pw, hwid) = self.read_credentials(packet)?;
        let acc = db::models::account::service::get_account_by_username(&user, ctx)
            .map_err(DatabaseError::from)
            .map_err(NetworkError::from)?;
        let login_action = {
            if self.authenticate(&acc, &pw)? {
                self.get_login_action(acc, hwid)?
            } else {
                LoginAction::RejectLogin {
                    acc: None,
                    reason: RejectReason::InvalidCredentials,
                }
            }
        };
        result.add_action(login_action);
        Ok(result)
    }
}
