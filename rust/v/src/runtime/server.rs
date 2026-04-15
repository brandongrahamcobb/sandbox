use crate::config::settings;
use crate::runtime::client::ActiveLogin;
use crate::runtime::error::RuntimeError;
use tokio::net::TcpStream;
use tracing::{error, warn};

pub struct LoginServer;

impl LoginServer {
    pub async fn run() -> Result<(), RuntimeError> {
        if let Ok(addr) = settings::get_login_server_addr() {
            let listener = tokio::net::TcpListener::bind(&addr).await?;
            loop {
                match listener.accept().await {
                    Ok((stream, addr)) => {
                        tokio::spawn(async move {
                            match ActiveLogin::new(stream, addr).await {
                                Ok(login) => login.run().await,
                                Err(e) => error!(error = %e, "Failed to create ActiveLogin"),
                            }
                        });
                    }
                    Err(e) => {
                        error!(error = %e, "Error accepting login connection");
                    }
                }
            }
        } else {
            Err(RuntimeError::Handler(String::from(
                "Expected a valid login port. Received a ConfigError.",
            )))
        }
    }
}
