use std::collections::HashMap;

use serenity::{
    async_trait,
    builder::CreateEmbed,
    model::prelude::{
        command::Command,
        interaction::{
            application_command::ApplicationCommandInteraction, Interaction,
            InteractionResponseType,
        },
        Message, Ready, ResumedEvent,
    },
    prelude::{Context, EventHandler},
};
use tracing::{debug, info, instrument};

use crate::commands::{
    self,
    birthday::{
        run_clear_command, run_info_command, run_remove_command, run_set_command,
        run_subscribe_command, run_unsubscribe_command,
    },
    CommandError,
};

pub struct Handler {
    pub database: sqlx::PgPool,
}

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            debug!("Received command interaction: {:#?}", command);

            if command.guild_id.is_none() {
                return;
            }

            let content = match command.data.name.as_str() {
                "birthday" => dispatch_birthday_sub_command(&command, &self.database).await,
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
    }

    #[instrument(skip(self, _ctx))]
    async fn resume(&self, _ctx: Context, resume: ResumedEvent) {
        debug!("Resumed; trace: {:?}", resume.trace);
    }
}

async fn dispatch_birthday_sub_command(
    command: &ApplicationCommandInteraction,
    database: &sqlx::PgPool,
) -> Result<CreateEmbed, CommandError> {
    let embed = CreateEmbed(HashMap::new())
        .title("Interaction failure")
        .description("Command has not been implemented.")
        .to_owned();

    if let Some(subcommand) = command.data.options.get(0) {
        return match subcommand.name.as_str() {
            "info" => run_info_command(&database, &command.guild_id.unwrap(), &command.user).await,
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
            "subscribe" => run_subscribe_command(
                &database,
                &command.guild_id.unwrap(),
                &command.user,
                &subcommand.options,
            ),
            "unsubscribe" => run_unsubscribe_command(
                &database,
                &command.guild_id.unwrap(),
                &command.user,
                &subcommand.options,
            ),
            "clear" => run_clear_command(
                &database,
                &command.guild_id.unwrap(),
                &command.user,
                &subcommand.options,
            ),
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
