use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};

use chrono::Datelike;
use serenity::{
    async_trait,
    builder::CreateEmbed,
    model::prelude::{
        command::Command,
        interaction::{
            application_command::ApplicationCommandInteraction, Interaction,
            InteractionResponseType,
        }, Message, Ready, ResumedEvent,
    },
    prelude::{Context, EventHandler},
};
use sqlx::{
    types::chrono::Utc,
    PgPool,
};
use tracing::{debug, error, info, instrument, warn};

use crate::{
    commands::{
        self,
        birthday::{
            run_info_command, run_remove_command, run_set_command, run_subscribe_command,
            run_unsubscribe_command,
        },
        CommandError,
    },
    models::{birthday::Birthday, subscription::{Subscription, SendNotification}},
};

pub struct Handler {
    pub database: sqlx::PgPool,
    pub is_loop_running: AtomicBool,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.starts_with("!ping") {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                eprintln!("Error sending message: {:?}", why);
            }
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            debug!("Received command interaction: {:#?}", command);

            if command.guild_id.is_none() {
                return;
            }

            let content = match command.data.name.as_str() {
                "birthday" => dispatch_birthday_sub_command(&command, &ctx, &self.database).await,
                _ => Ok(CreateEmbed(HashMap::new())
                    .title("Interaction failure")
                    .description("Command has not been implemented.")
                    .to_owned()),
            };

            let embed = match content {
                Ok(e) => e,
                Err(why) => {
                    tracing::error!("Cannot respond to slash command: {:?}", why);
                    CreateEmbed(HashMap::new())
                        .title("Interaction failure")
                        .description("Command ran into an error.")
                        .to_owned()
                }
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.add_embed(embed))
                })
                .await
            {
                tracing::error!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        let guild_command = Command::create_global_application_command(&ctx.http, |command| {
            commands::birthday::register(command)
        })
        .await;

        debug!(
            "I created the following global slash command: {:#?}",
            guild_command
        );

        let ctx = Arc::new(ctx);
        let db = Arc::new(self.database.clone());

        if !self.is_loop_running.load(Ordering::Relaxed) {
            let ctx3 = Arc::clone(&ctx);
            let db1 = Arc::clone(&db);

            tokio::spawn(async move {
                loop {
                    if let Err(why) =
                        notify_birthdays(Arc::clone(&ctx3), Arc::clone(&db1)).await
                    {
                        error!("Failed to notify birthdays, err: {}", why);
                    };
                    tokio::time::sleep(Duration::from_secs(60)).await;
                }
            });

            self.is_loop_running.swap(true, Ordering::Relaxed);
        }
    }

    #[instrument(skip(self, _ctx))]
    async fn resume(&self, _ctx: Context, resume: ResumedEvent) {
        debug!("Resumed; trace: {:?}", resume.trace);
    }
}

async fn notify_birthdays(
    ctx: Arc<Context>,
    db: Arc<PgPool>,
) -> Result<(), sqlx::Error> {
    info!("Notification started!");

    let today = Utc::now().naive_utc().date();

    let birthdays = Birthday::get_all(&db).await?;
    let birthdays = birthdays
        .iter()
        .filter(|b| b.date.date() == today);

    for birthday in birthdays {
        if let Ok(bday_user) = ctx.http.get_user(birthday.user_id()).await {
            let subscriptions =
                Subscription::get_all_by_birthday_id(&db, birthday.id_birthday, today.year()).await?;

            send_birthday_dm(subscriptions, &ctx, &db, &bday_user.name).await;
        }
        else {
            warn!("Could not find user: {}", birthday.user_id());
        }
    }

    info!("Notification finished!");

    Ok(())
}

async fn send_birthday_dm(
    subscriptions: Vec<Subscription>,
    ctx: &Arc<Context>,
    db: &Arc<PgPool>,
    user_name: &str,
) {
    let today = Utc::now().naive_utc();

    for subscription in subscriptions {
        if let Ok(user) = ctx.http.get_user(subscription.user_id()).await {
            if let Ok(priv_channel) = user.create_dm_channel(ctx).await {
                if let Err(why) = priv_channel
                    .send_message(&ctx.http, |message| {
                        message.embed(|embed| {
                            embed.title("Birthday:").description(format!(
                                "Hey the user `{}` has birthday today ({}).",
                                user_name, today,
                            ))
                        })
                    })
                    .await
                {
                    error!("Could not send birthday in dm channel, err: {}", why);
                } else {
                    
                    let mut send_notification = SendNotification::new(subscription.id_subscription, today.year(), today);
                    
                    match send_notification.insert(&db).await {
                        Ok(_) => info!("Notified of birthday!"),
                        Err(why) => error!("Could not create notifcation, why: {why}"),
                    };
                }
            }
        }
    }
}

async fn dispatch_birthday_sub_command(
    command: &ApplicationCommandInteraction,
    ctx: &Context,
    database: &sqlx::PgPool,
) -> Result<CreateEmbed, CommandError> {
    let embed = CreateEmbed(HashMap::new())
        .title("Interaction failure")
        .description("Command has not been implemented.")
        .to_owned();

    if let Some(subcommand) = command.data.options.get(0) {
        return match subcommand.name.as_str() {
            "info" => {
                run_info_command(&database, &ctx, &command.guild_id.unwrap(), &command.user).await
            }
            "set" => {
                run_set_command(
                    &database,
                    &command.guild_id.unwrap(),
                    &command.user,
                    &subcommand.options,
                )
                .await
            }
            "remove" => {
                run_remove_command(&database, &command.guild_id.unwrap(), &command.user).await
            }
            "subscribe" => {
                run_subscribe_command(
                    &database,
                    &command.guild_id.unwrap(),
                    &command.user,
                    &subcommand.options,
                )
                .await
            }
            "unsubscribe" => {
                run_unsubscribe_command(
                    &database,
                    &command.guild_id.unwrap(),
                    &command.user,
                    &subcommand.options,
                )
                .await
            }
            _ => Ok(embed),
        };
    }

    Ok(embed)
}

#[instrument]
pub async fn before(_: &Context, msg: &Message, command_name: &str) -> bool {
    info!(
        "Got command '{}' by user '{}'",
        command_name, msg.author.name
    );

    true
}
