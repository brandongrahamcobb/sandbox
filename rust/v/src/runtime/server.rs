use crate::config::settings;
use crate::runtime::error::RuntimeError;
use crate::runtime::relay::{Credentials, Runtime, World};
use crate::runtime::state::SharedState;
use tracing::info;

pub enum ServerType {
    LoginServer,
    WorldServer,
}

pub struct LoginServer;

impl LoginServer {
    pub async fn run(shared_state: &SharedState) -> Result<(), RuntimeError> {
        if let Ok(addr) = settings::get_login_server_addr(&shared_state.settings) {
            let listener = tokio::net::TcpListener::bind(&addr).await?;
            loop {
                match listener.accept().await {
                    Ok((stream, _addr)) => {
                        let shared_state = shared_state.clone();
                        tokio::spawn(async move {
                            match Runtime::<Credentials>::new(shared_state, stream).await {
                                Ok(mut cred) => {
                                    if let Err(e) = cred.run().await {
                                        info!(
                                            "Expected a successful login relay loop. Received an error. Error: {}",
                                            e.to_string(),
                                        );
                                    }
                                }
                                Err(e) => info!(
                                    "Expected valid login relay creation. Received an error. Error: {}",
                                    e.to_string(),
                                ),
                            };
                        });
                    }
                    Err(e) => info!(
                        "Expected valid connection. Received an error. Error: {}",
                        e.to_string(),
                    ),
                }
            }
        } else {
            Err(RuntimeError::UnexpectedError)
        }
    }
}

pub struct WorldServer;

impl WorldServer {
    pub async fn run(shared_state: &SharedState) -> Result<(), RuntimeError> {
        if let Ok(addr) = settings::get_world_server_addr(&shared_state.settings) {
            let listener = tokio::net::TcpListener::bind(&addr).await?;
            loop {
                match listener.accept().await {
                    Ok((stream, _addr)) => {
                        let shared_state = shared_state.clone();
                        tokio::spawn(async move {
                            match Runtime::<World>::new(shared_state, stream).await {
                                Ok(mut world) => {
                                    if let Err(e) = world.run().await {
                                        info!(
                                            "Expected a successful world relay loop. Received an error. Error: {}",
                                            e.to_string(),
                                        );
                                    }
                                }
                                Err(e) => info!(
                                    "Expected valid world relay creation. Received an error. Error: {}",
                                    e.to_string(),
                                ),
                            };
                        });
                    }
                    Err(e) => info!(
                        "Expected valid connection. Received an error. Error: {}",
                        e.to_string(),
                    ),
                }
            }
        } else {
            Err(RuntimeError::UnexpectedError)
        }
    }
}
