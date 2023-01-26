use std::collections::HashMap;

use serenity::builder::{CreateApplicationCommand, CreateEmbed};
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use serenity::model::prelude::{Embed, GuildId};
use serenity::model::user::User;
use sqlx::PgPool;

use crate::models::birthday::Birthday;
use crate::models::subscription::Subscription;

pub async fn run_info_command(
    db: &PgPool,
    guild_id: &GuildId,
    user: &User,
    _options: &[CommandDataOption],
) -> Result<CreateEmbed, sqlx::Error> {
    if let Some(bday) = Birthday::get(db, guild_id.0, user.id.0).await? {
        let subscriptions = Subscription::get_all(db, guild_id.0, user.id.0).await?;

        let embed = CreateEmbed(HashMap::new())
            .title(format!("Birthday info for: <@{}>", user.id))
            .description(format!("Birthday: {}", bday.date));
    }

    let embed = CreateEmbed(HashMap::new())
        .title("Interaction failure")
        .description("You have not registered your birthday yet.")
        .to_owned();

    Ok(embed)
}

pub fn run_set_command(
    db: &PgPool,
    guild_id: &GuildId,
    user: &User,
    _options: &[CommandDataOption],
) -> Result<CreateEmbed, sqlx::Error> {


    let embed = CreateEmbed(HashMap::new())
        .title("Interaction test")
        .description(format!(
            "Set command from guild: {}, user: {}",
            guild_id, user.id
        ))
        .to_owned();

    Ok(embed)
}

pub fn run_remove_command(
    db: &PgPool,
    guild_id: &GuildId,
    user: &User,
    _options: &[CommandDataOption],
) -> Result<CreateEmbed, sqlx::Error> {
    let embed = CreateEmbed(HashMap::new())
        .title("Interaction test")
        .description(format!(
            "Remove command from guild: {}, user: {}",
            guild_id, user.id
        ))
        .to_owned();

    Ok(embed)
}

pub fn run_subscribe_command(
    db: &PgPool,
    guild_id: &GuildId,
    user: &User,
    _options: &[CommandDataOption],
) -> Result<CreateEmbed, sqlx::Error> {
    let embed = CreateEmbed(HashMap::new())
        .title("Interaction test")
        .description(format!(
            "Subscribe command from guild: {}, user: {}",
            guild_id, user.id
        ))
        .to_owned();

    Ok(embed)
}

pub fn run_unsubscribe_command(
    db: &PgPool,
    guild_id: &GuildId,
    user: &User,
    _options: &[CommandDataOption],
) -> Result<CreateEmbed, sqlx::Error> {
    let embed = CreateEmbed(HashMap::new())
        .title("Interaction test")
        .description(format!(
            "Unsubscribe command from guild: {}, user: {}",
            guild_id, user.id
        ))
        .to_owned();

    Ok(embed)
}

pub fn run_clear_command(
    db: &PgPool,
    guild_id: &GuildId,
    user: &User,
    _options: &[CommandDataOption],
) -> Result<CreateEmbed, sqlx::Error> {
    let embed = CreateEmbed(HashMap::new())
        .title("Interaction test")
        .description(format!(
            "Remove command from guild: {}, user: {}",
            guild_id, user.id
        ))
        .to_owned();

    Ok(embed)
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    build_info_command(command);
    build_set_command(command);
    build_remove_command(command);
    build_subscribe_command(command);
    build_unsubscribe_command(command);
    build_clear_command(command)
}

fn build_info_command(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("birthday")
        .description("A command for birthdays.")
        .create_option(|sub_command| {
            sub_command
                .name("info")
                .description("Gets the birthday of a user.")
                .kind(CommandOptionType::SubCommand)
        })
}

fn build_set_command(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("birthday")
        .description("A command for birthdays.")
        .create_option(|sub_command| {
            sub_command
                .name("set")
                .description("Gets the birthday of a user.")
                .kind(CommandOptionType::SubCommand)
                    .create_sub_option(|option| {
                        option
                            .name("day")
                            .description("The day you were born.")
                            .kind(CommandOptionType::Integer)
                    })
                    .create_sub_option(|option| {
                        option
                            .name("month")
                            .description("The month you were born.")
                            .kind(CommandOptionType::Integer)
                    })
                    .create_sub_option(|option| {
                        option
                            .name("year")
                            .description("The year you were born.")
                            .kind(CommandOptionType::Integer)
                    })
        })
}

fn build_remove_command(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("birthday")
        .description("A command for birthdays.")
        .create_option(|sub_command| {
            sub_command
                .name("remove")
                .description("Gets the birthday of a user.")
                .kind(CommandOptionType::SubCommand)
        })
}

fn build_subscribe_command(
    command: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand {
    command
        .name("birthday")
        .description("A command for birthdays.")
        .create_option(|sub_command| {
            sub_command
                .name("subscribe")
                .description("Gets the birthday of a user.")
                .kind(CommandOptionType::SubCommand)
        })
}

fn build_unsubscribe_command(
    command: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand {
    command
        .name("birthday")
        .description("A command for birthdays.")
        .create_option(|sub_command| {
            sub_command
                .name("unsubscribe")
                .description("Gets the birthday of a user.")
                .kind(CommandOptionType::SubCommand)
        })
}

fn build_clear_command(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("birthday")
        .description("A command for birthdays.")
        .create_option(|sub_command| {
            sub_command
                .name("clear")
                .description("Gets the birthday of a user.")
                .kind(CommandOptionType::SubCommand)
        })
}
