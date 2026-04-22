use crate::db::pool::DbPool;
use crate::runtime::session::SessionStore;
use config::Config;
use std::sync::Arc;

pub struct State {
    pub db: DbPool,
    pub settings: Config,
    pub sessions: SessionStore,
}

pub type SharedState = Arc<State>;
