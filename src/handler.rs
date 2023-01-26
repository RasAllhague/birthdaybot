use std::env;

use serenity::{
    async_trait,
    model::prelude::{
        command::Command,
        interaction::{
            application_command::ApplicationCommandInteraction, Interaction,
            InteractionResponseType,
        },
        GuildId, Message, Ready, ResumedEvent,
    },
    prelude::{Context, EventHandler},
};
use tracing::{debug, info, instrument};

use crate::commands::{
    self,
    birthday::{run_clear, run_info, run_remove, run_set, run_subscribe, run_unsubscribe},
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            info!("Received command interaction: {:#?}", command);

            if command.guild_id.is_none() {
                return;
            }

            let content = match command.data.name.as_str() {
                "birthday" => dispatch_birthday_sub_command(&command),
                _ => "not implemented :(".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                info!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        let guild_command = Command::create_global_application_command(&ctx.http, |command| {
            commands::birthday::register(command)
        })
        .await;

        info!(
            "I created the following global slash command: {:#?}",
            guild_command
        );
    }

    #[instrument(skip(self, _ctx))]
    async fn resume(&self, _ctx: Context, resume: ResumedEvent) {
        debug!("Resumed; trace: {:?}", resume.trace);
    }
}

fn dispatch_birthday_sub_command(command: &ApplicationCommandInteraction) -> String {
    if let Some(subcommand) = command.data.options.get(0) {
        return match subcommand.name.as_str() {
            "info" => run_info(
                &command.guild_id.unwrap(),
                &command.user,
                &subcommand.options,
            ),
            "set" => run_set(
                &command.guild_id.unwrap(),
                &command.user,
                &subcommand.options,
            ),
            "remove" => run_remove(
                &command.guild_id.unwrap(),
                &command.user,
                &subcommand.options,
            ),
            "subscribe" => run_subscribe(
                &command.guild_id.unwrap(),
                &command.user,
                &subcommand.options,
            ),
            "unsubscribe" => run_unsubscribe(
                &command.guild_id.unwrap(),
                &command.user,
                &subcommand.options,
            ),
            "clear" => run_clear(
                &command.guild_id.unwrap(),
                &command.user,
                &subcommand.options,
            ),
            _ => "Not implemented".to_string(),
        };
    }

    "Not implemented".to_string()
}

#[instrument]
pub async fn before(_: &Context, msg: &Message, command_name: &str) -> bool {
    info!(
        "Got command '{}' by user '{}'",
        command_name, msg.author.name
    );

    true
}
