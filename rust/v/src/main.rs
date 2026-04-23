use std::sync::Arc;
use tracing::{error, info};
use tracing_subscriber::EnvFilter;
use v::config::settings;
use v::db::pool;
use v::net::world;
use v::runtime::error::RuntimeError;
use v::runtime::server::{LoginServer, WorldServer};
use v::runtime::session::SessionStore;
use v::runtime::state::{SharedState, State};

#[tokio::main]
async fn main() -> Result<(), RuntimeError> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::from_default_env().add_directive("server=info".parse().unwrap()),
        )
        .init();
    info!("Loading shared state...");
    let shared_state: SharedState = Arc::new(State::new()?);
    info!("Starting Login Server...");
    LoginServer::run(&shared_state.clone()).await?;
    info!("Starting World Server...");
    WorldServer::run(&shared_state.clone()).await?;
    Ok(())
}
