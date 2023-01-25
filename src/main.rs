use std::env;

use handler::{before, Handler};
use serenity::{
    framework::{
        standard::{
            macros::{command, group},
            CommandResult,
        },
        StandardFramework,
    },
    model::prelude::{command, Message},
    prelude::{Context, GatewayIntents},
    Client,
};
use tracing::{error, instrument};

mod handler;

#[group]
#[commands(ping)]
struct General;

#[tokio::main]
#[instrument]
async fn main() {
    tracing_subscriber::fmt::init();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~"))
        .before(before)
        .group(&GENERAL_GROUP);

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    if let Err(why) = msg.channel_id.say(&ctx.http, "Pong! : )").await {
        error!("Error sending message: {:?}", why);
    }

    Ok(())
}
