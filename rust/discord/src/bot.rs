use config::Config;
use serde::Serialize;

use crate::{
    client::Client, config::settings, error::DiscordError, gateway::gateway::PresenceUpdate,
    logger::setup_logger, response::UserResponse,
};

#[derive(Debug, Serialize)]
pub struct ClientBuilderWS {
    pub intents: Option<i32>,
    pub reconnect: Option<bool>,
    pub shard: usize,
    pub presence: Option<PresenceUpdate>,
}
pub struct BotBase {
    pub intents: Option<i32>,
    pub client: Option<Client>,
    pub presence: Option<PresenceUpdate>,
    pub settings: Option<Config>,
}

impl BotBase {
    pub async fn new(intents: Option<i32>) -> Self {
        log::debug!("🤖 Creating new BotBase instance");
        if let Some(intents) = intents {
            log::debug!("🎯 Bot intents configured: {}", intents);
        } else {
            log::debug!("🎯 Bot intents not specified, will use defaults");
        }
        BotBase {
            intents: intents,
            client: None,
            presence: None,
            settings: None,
        }
    }

    /// login the bot
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rustycord::bot::BotBase;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let mut bot_base = BotBase::new(None).await;
    /// let result = bot_base.login("your_token_here".to_string()).await;
    /// println!("Login result: {:?}", result);
    /// # }
    /// ```
    pub async fn login(&mut self, token: String) -> UserResponse {
        log::info!("🔑 Initializing bot login...");
        let mut _client = Client::new(
            <std::option::Option<Config> as Clone>::clone(&self.settings).expect("ConfigError"),
        );
        let res = _client.login(token).await;
        log::info!("🔒 Logged in as: {:?}", res.username);
        self.client = Some(_client);
        res
    }

    /// start the bot
    ///
    /// # Arguments
    ///
    /// * `token` - the token of the bot
    /// * `reconnect` - if the bot should reconnect
    ///
    pub async fn start(&mut self, token: String, reconnect: Option<bool>) -> () {
        log::info!("🚀 Starting bot...");
        self.login(token).await;
        log::info!("📡 Establishing WebSocket connection...");
        self.connect(self.intents, reconnect).await;
    }

    pub async fn stop(&self) -> bool {
        true
    }

    pub async fn connect(&mut self, intents: Option<i32>, reconnect: Option<bool>) {
        log::debug!("🌐 Connecting to Discord gateway...");
        self.client
            .as_mut()
            .unwrap()
            .ws_connect(intents, reconnect, None)
            .await;
        log::info!("✅ Successfully connected to Discord gateway");
    }

    pub async fn set_presence(&mut self, presence: PresenceUpdate) {
        self.presence = Some(presence);
    }

    // pub async fn set_shard(&mut self, shard: usize) {
    //     let ws = ClientBuilderWS {
    //         intents: self.intents,
    //         reconnect: Some(true),
    //         shard: shard,
    //         presence: self.presence,
    //     };
    // }
    pub async fn run(&mut self) -> Result<&mut Self, DiscordError> {
        let settings = settings::get_settings()?;
        let token = settings::get_token(&settings)?;
        let logging_level = settings::get_logging_level(&settings)?;
        self.settings = Some(settings);
        log::info!("⚙️ Initializing logger with level: {}", logging_level);
        setup_logger(logging_level);
        log::info!("🤖 rustycord bot starting up...");
        self.start(token, Some(true)).await;
        Ok(self)
    }
}

pub struct Bot {}

impl Bot {
    pub async fn builder(intents: Option<i32>) -> BotBase {
        let bot = BotBase::new(intents).await;
        bot
    }
}
