use crate::db::models::account::core::Account;

pub enum RejectReason {
    Banned,
    PendingTOS,
    Playing,
    InvalidCredentials,
}

pub enum LoginAction {
    RejectLogin {
        acc: Option<Account>,
        reason: RejectReason,
    },
    AcceptLogin {
        acc: Account,
        hwid: String,
    },
}
