use std::sync::Arc;
use tracing::info;
use tracing_subscriber::EnvFilter;
use v::runtime::error::RuntimeError;
use v::runtime::server::{LoginServer, WorldServer};
use v::runtime::state::{SharedState, State};

#[tokio::main]
async fn main() -> Result<(), RuntimeError> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("v=info".parse().unwrap()))
        .init();
    info!("Loading Shared State...");
    let shared_state: SharedState = Arc::new(State::new()?);
    info!("Starting Login Server...");
    let login = LoginServer::run(&shared_state); //.await?;
    info!("Starting World Server...");
    let world = WorldServer::run(&shared_state); //.await?;
    tokio::try_join!(login, world)?;
    Ok(())
}
