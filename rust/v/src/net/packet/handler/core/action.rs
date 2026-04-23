use crate::db::models::account::core::Account;
//use crate::runtime::session::SessionState;
use crate::net::packet::core::Packet;

pub enum RejectLoginReason {
    Banned,
    PendingTOS,
    Playing,
    InvalidCredentials,
}

pub enum CoreAction {
    RejectLogin {
        acc: Option<Account>,
        reason: RejectLoginReason,
    },
    AcceptLogin {
        acc: Account,
        hwid: String,
    },
    Simple {
        packet: Packet,
    }, // CreateSession {
       //     acc: Account,
       //     hwid: String,
       // },
       // SessionSelectWorld {
       //     session_state: SessionState,
       //     world_id: u8,
       // },
       // SessionSelectChannel {
       //     session_state: SessionState,
       //     channel_id: u8,
       // },
}
