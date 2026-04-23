use std::sync::Arc;
use tracing::info;
use tracing_subscriber::EnvFilter;
use v::runtime::error::RuntimeError;
use v::runtime::server::{LoginServer, WorldServer};
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
