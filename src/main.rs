use std::env;

use handler::Handler;
use serenity::{
    prelude::GatewayIntents,
    Client,
};
use tracing::{error, instrument};

mod handler;
mod commands;

#[tokio::main]
#[instrument]
async fn main() {
    tracing_subscriber::fmt::init();

    let token = env::var("BIRTHDAY_BOT_TOKEN").expect("Expected a token in the environment");

    let intents = GatewayIntents::empty();
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}