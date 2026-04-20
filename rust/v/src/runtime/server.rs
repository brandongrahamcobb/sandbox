use crate::config::settings;
use crate::runtime::error::RuntimeError;
use crate::runtime::relay::{Authentication, World};
use tokio::net::TcpStream;
use tracing::{error, warn};

pub enum ServerType {
    AuthenticationServer,
    WorldServer,
}

pub struct AuthenticationServer;

impl AuthenticationServer {
    pub async fn run() -> Result<(), RuntimeError> {
        if let Ok(addr) = settings::get_login_server_addr() {
            let listener = tokio::net::TcpListener::bind(&addr).await?;
            loop {
                match listener.accept().await {
                    Ok((stream, addr)) => {
                        tokio::spawn(async move {
                            match Login::new(stream, addr).await {
                                Ok(login) => login.run().await,
                                Err(e) => Err(RuntimeError::GenericRuntimeError(
                                    "Failed to create Authentication. Error: {}",
                                    e.to_string(),
                                )),
                            }
                        });
                    }
                    Err(e) => Err(RuntimeError::GenericRuntimeError(
                        "Error accepting authentication connection. Error: {}",
                        e.to_string(),
                    )),
                }
            }
        } else {
            Err(RuntimeError::RuntimeConfigError(
                "Expected a valid login SocketAddr. Received a ConfigError.",
            ))
        }
    }
}

pub struct WorldServer;

impl WorldServer {
    pub async fn run() -> Result<(), RuntimeError> {
        if let Ok(addr) = settings::get_world_server_addr() {
            let listener = tokio::net::TcpListener::bind(&addr).await?;
            loop {
                match listener.accept().await {
                    Ok((stream, peer_addr)) => {
                        tokio::spawn(async move {
                            match World::new(stream, peer_addr).await {
                                Ok(world) => world.run().await,
                                Err(e) => Err(RuntimeError::GenericRuntimeError(
                                    "Failed to create World. Error: {}",
                                    e.to_string(),
                                )),
                            }
                        });
                    }
                    Err(e) => Err(RuntimeError::GenericRuntimeError(
                        "Error accepting world connection. Error: {}",
                        e.to_string(),
                    )),
                }
            }
        } else {
            Err(RuntimeError::RuntimeConfigError(
                "Expected a valid world SocketAddr. Received a ConfigError.",
            ));
        }
    }
}
