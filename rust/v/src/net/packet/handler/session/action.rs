use crate::{db::models::account::core::Account, runtime::session::SessionState};

pub enum SessionAction {
    CreateSession {
        acc: Account,
        hwid: String,
    },
    SessionSelectWorld {
        session_state: SessionState,
        world_id: u8,
    },
    SessionSelectChannel {
        session_state: SessionState,
        channel_id: u8,
    },
}
