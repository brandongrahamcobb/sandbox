use crate::{db::models::account::core::Account, runtime::session::LoginSessionState};

pub enum SessionAction {
    CreateSession {
        acc: Account,
        hwid: String,
    },
    SessionSelectWorld {
        state: LoginSessionState,
        world_id: u8,
    },
    SessionSelectChannel {
        state: LoginSessionState,
        channel_id: u8,
    },
}
