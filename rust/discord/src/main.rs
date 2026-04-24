// main.rs or your entry point

use discord::bot::BotBase;
use discord::cog::MyHandler;
use discord::config::settings;
use discord::error::DiscordError;
use discord::gateway::intents;
use discord::{client::Client, logger::setup_logger};

#[tokio::main]
async fn main() -> Result<(), DiscordError> {
    // Initialize logging (optional)
    dotenvy::dotenv();

    let intents = intents::ALL_INTENTS;
    let mut bot_base: BotBase = BotBase::new(Some(intents)).await;
    let result = bot_base.run().await.expect("DiscordError");
    // Create a new client
    let mut client = result.client.as_mut().expect("client not initialized");
    let handlers = client.get_event_dispatcher().get_message_handlers();
    handlers.add_handler(MyHandler).await;

    let gateway_url = client.get_gateway().await;
    log::info!("Gateway URL: {}", gateway_url);

    // Determine the number of shards
    // You can fetch this from the gateway/bot endpoint or set it manually
    let total_shards = 2; // Example: 2 shards

    // Initialize shards
    client.initialize_shards(total_shards).await;

    // Start shards
    client.start_shards(Some(513), None).await;

    // Keep the main task alive
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
    }
}
