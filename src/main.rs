use std::env;

use handler::{before, Handler};
use serenity::{framework::{StandardFramework, standard::{macros::{group, command}, CommandResult}}, model::prelude::{command, Message}, prelude::{Context, GatewayIntents}, Client};
use tracing::{error, instrument};

mod handler;

#[group]
#[commands(ping)]
struct General;

#[tokio::main]
#[instrument]
async fn main() {
    // Call tracing_subscriber's initialize function, which configures `tracing`
    // via environment variables.
    //
    // For example, you can say to log all levels INFO and up via setting the
    // environment variable `RUST_LOG` to `INFO`.
    //
    // This environment variable is already preset if you use cargo-make to run
    // the example.
    tracing_subscriber::fmt::init();

    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let framework =
        StandardFramework::new().configure(|c| c.prefix("~")).before(before).group(&GENERAL_GROUP);

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

// Currently, the instrument macro doesn't work with commands.
// if you wish to instrument commands, use it on the before function.
#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    if let Err(why) = msg.channel_id.say(&ctx.http, "Pong! : )").await {
        error!("Error sending message: {:?}", why);
    }

    Ok(())
}