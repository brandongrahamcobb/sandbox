use crate::config::settings;
use crate::db::error::DatabaseError;
use crate::db::pool::DbPool;
use crate::net::world;
use crate::net::world::core::World;
use crate::runtime::error::RuntimeError;
use crate::runtime::session::SessionStore;
use config::Config;
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use std::sync::Arc;
use tracing::info;

pub struct State {
    pub db: DbPool,
    pub settings: Config,
    pub sessions: SessionStore,
    pub worlds: Vec<World>,
}

pub type SharedState = Arc<State>;

impl State {
    pub fn new() -> Result<Self, RuntimeError> {
        info!("Loading settings...");
        let settings = settings::get_settings().map_err(RuntimeError::from)?;
        let db_url = settings::get_db_url(&settings)?;
        let manager = ConnectionManager::<PgConnection>::new(db_url);
        let db = Pool::builder()
            .build(manager)
            .map_err(|_| RuntimeError::from(DatabaseError::DatabaseConnectionError))?;
        info!("Loading sessions...");
        let sessions = SessionStore::new();
        info!("Loading worlds...");
        let worlds = world::core::load_worlds(&settings)?;
        let shared_state = State {
            db,
            settings,
            sessions,
            worlds,
        };
        Ok(shared_state)
    }
}
