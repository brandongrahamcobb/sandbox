use crate::client::Client;
use crate::config::settings;
use crate::error::DiscordError;
use crate::handlers::message_handler::{MessageHandler, MessageHandlerResult};
use crate::message::ChannelMessage;
use crate::prelude::*;
use async_trait::async_trait;
use config::Config;

pub struct MyHandler;

#[async_trait]
impl MessageHandler for MyHandler {
    async fn on_message_create(
        self: &Self,
        message: &ChannelMessage,
        client: &Client,
    ) -> MessageHandlerResult {
        if message.author.bot.unwrap_or(false) {
            return Ok(());
        }
        if message.content == "!test" {
            if let Ok(addr) = settings::get_core_server_addr(
                &<std::option::Option<Config> as Clone>::clone(&client.settings)
                    .expect("ConfigError"),
            )
            .map_err(DiscordError::from)
            {
                let mut stream = tokio::net::TcpStream::connect(&addr).await?;
                stream.write_all(b"Hello").await?;
            }
        }
        Ok(())
    }
}
