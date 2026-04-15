use tracing::{error, info};
use tracing_subscriber::EnvFilter;
use v::runtime::server::LoginServer;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::from_default_env().add_directive("server=info".parse().unwrap()),
        )
        .init();

    info!("Starting Login Server...");
    if let Err(e) = LoginServer::run().await {
        error!(error = %e, "Login server error");
    }
}
