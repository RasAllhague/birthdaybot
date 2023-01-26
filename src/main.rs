use std::env;

use handler::Handler;
use serenity::{prelude::GatewayIntents, Client};
use tracing::{error, instrument};

mod commands;
mod handler;
mod models;

#[tokio::main]
#[instrument]
async fn main() {
    tracing_subscriber::fmt::init();

    let token = env::var("BIRTHDAY_BOT_TOKEN").expect("Expected a token in the environment");
    let db_url = env::var("DATABASE_URL").expect("Expected database url in the environment");

    let database = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Couldn't connect to database");

    sqlx::migrate!("./migrations")
        .run(&database)
        .await
        .expect("Couldn't run database migrations");

    let intents = GatewayIntents::empty();
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler { database })
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
