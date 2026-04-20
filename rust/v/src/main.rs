use tracing::{error, info};
use tracing_subscriber::EnvFilter;
use v::db::db;
use v::runtime::server::{AuthenticationServer, WorldServer};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::from_default_env().add_directive("server=info".parse().unwrap()),
        )
        .init();
    info!("Starting Authentication Server...");
    if let Err(e) = AuthenticationServer::run().await {
        error!("Authentication server error. Error: {}", e.to_string());
    }
    info!("Starting World Server...");
    if let Err(e) = WorldServer::run().await {
        error!("World server error. Error: {}", e.to_string());
    }
    info!("Starting Database...");
    if let Err(e) = db::establish_connection() {
        error!("Database connection error. Error: {}", e.to_string());
    }
}
