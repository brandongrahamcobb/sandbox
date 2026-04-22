use std::sync::Arc;
use tracing::{error, info};
use tracing_subscriber::EnvFilter;
use v::config::settings;
use v::db::pool;
use v::runtime::server::{LoginServer, WorldServer};
use v::runtime::session::SessionStore;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::from_default_env().add_directive("server=info".parse().unwrap()),
        )
        .init();
    info!("Loading settings...");
    let settings = settings::get_settings()?;
    info!("Starting Database...");
    let db = db::establish_pool()?;
    let session = SessionStore::new();
    let state = Arc::new(State {
        db,
        settings,
        session,
    });
    info!("Starting Login Server...");
    LoginServer::run(state.clone()).await?;
    info!("Starting World Server...");
    WorldServer::run(state.clone()).await?;
}
