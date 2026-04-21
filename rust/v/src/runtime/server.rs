use crate::config::settings;
use crate::runtime::error::{
    RuntimeError, RuntimeRelayCreationError, RuntimeServerConnectionError,
};
use crate::runtime::relay::{Credentials, Runtime, World};
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
                            match Runtime::<Credentials>::new(stream, addr).await {
                                Ok(cred) => {
                                    if let Err(e) = cred.run().await {
                                        !info(
                                            "Expected a successful login relay. Received an error. Error: {}",
                                            e.to_string(),
                                        );
                                    }
                                }
                                Err(e) => Err(RuntimeRelayCreationError::FailedLoginRelayCreation(
                                    e.to_string,
                                )),
                            };
                        });
                    }
                    Err(e) => Err(RuntimeServerConnectionError::FailedLoginServerConnection(
                        e.to_string(),
                    )),
                }
            }
        } else {
            Err(RuntimeError::RuntimeConfigError)
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
                            match Runtime::<World>::new(stream, addr).await {
                                Ok(world) => {
                                    if let Err(e) = world.run().await {
                                        !info(
                                            "Expected a successful world relay. Received an error. Error: {}",
                                            e.to_string(),
                                        );
                                    }
                                }
                                Err(e) => Err(RuntimeRelayCreationError::FailedWorldRelayCreation(
                                    e.to_string,
                                )),
                            };
                        });
                    }
                    Err(e) => Err(RuntimeServerConnectionError::FailedWorldServerConnection(
                        e.to_string(),
                    )),
                }
            }
        } else {
            Err(RuntimeError::RuntimeConfigError)
        }
    }
}
