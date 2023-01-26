use std::collections::HashMap;

use serenity::builder::{CreateApplicationCommand, CreateEmbed};
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};
use serenity::model::prelude::{Embed, GuildId};
use serenity::model::user::User;
use sqlx::types::chrono::{NaiveDate, NaiveDateTime, Utc};
use sqlx::PgPool;
use tracing::info;

use crate::models::birthday::{self, Birthday};
use crate::models::subscription::Subscription;
use crate::utils;

use super::parser::DateInputParser;
use super::CommandError;

pub async fn run_info_command(
    db: &PgPool,
    guild_id: &GuildId,
    user: &User,
) -> Result<CreateEmbed, CommandError> {
    info!("Gid: {}, uid: {}", guild_id, user.id);

    if let Some(bday) = Birthday::get(db, guild_id.0, user.id.0)
        .await
        .map_err(|x| CommandError::Db(x))?
    {
        let subscriptions = Subscription::get_all(db, guild_id.0, user.id.0)
            .await
            .map_err(|x| CommandError::Db(x))?;

        let embed = CreateEmbed(HashMap::new())
            .title("Birthday:")
            .description(format!("{}", bday.date.date()))
            .author(|author| {
                author
                    .name(user.name.clone())
                    .icon_url(utils::get_icon_url(user))
            })
            .to_owned();

        return Ok(embed);
    }

    let embed = CreateEmbed(HashMap::new())
        .title("Interaction failure")
        .description("You have not registered your birthday yet.")
        .to_owned();

    Ok(embed)
}

pub async fn run_set_command(
    db: &PgPool,
    guild_id: &GuildId,
    user: &User,
    options: &[CommandDataOption],
) -> Result<CreateEmbed, CommandError> {
    let date_parser = DateInputParser;
    let date = date_parser
        .parse(options)
        .map_err(|x| CommandError::Parser(x))?;

    let mut text_part = "set";
    let mut birthday: Birthday;

    if let Some(mut bday) = Birthday::get(db, guild_id.0, user.id.0)
        .await
        .map_err(|x| CommandError::Db(x))?
    {
        bday.date = date;
        bday.modify_date = Some(Utc::now().naive_utc());
        bday.update(db).await.map_err(|x| CommandError::Db(x))?;

        text_part = "updated";
        birthday = bday;
    } else {
        birthday = Birthday::new(guild_id.0, user.id.0, date, Utc::now().naive_utc());
        birthday.insert(db).await.map_err(|x| CommandError::Db(x))?;
    }

    let embed = CreateEmbed(HashMap::new())
        .title("Birthday:")
        .description(format!(
            "Birthday has been {} to: {}",
            text_part,
            birthday.date.date()
        ))
        .author(|author| {
            author
                .name(user.name.clone())
                .icon_url(utils::get_icon_url(user))
        })
        .to_owned();

    Ok(embed)
}

pub async fn run_remove_command(
    db: &PgPool,
    guild_id: &GuildId,
    user: &User,
) -> Result<CreateEmbed, CommandError> {
    if let Some(birthday) = Birthday::get(db, guild_id.0, user.id.0)
        .await
        .map_err(|x| CommandError::Db(x))?
    {
        birthday.delete(db).await.map_err(|x| CommandError::Db(x))?;

        let embed = CreateEmbed(HashMap::new())
            .title("Birthday:")
            .description("Your birthday and all subscriptions to this birthday has been deleted.")
            .author(|author| {
                author
                    .name(user.name.clone())
                    .icon_url(utils::get_icon_url(user))
            })
            .to_owned();

        return Ok(embed);
    }

    let embed = CreateEmbed(HashMap::new())
        .title("Birthday:")
        .description("You currently have no birthday set up, which i could delete!")
        .author(|author| {
            author
                .name(user.name.clone())
                .icon_url(utils::get_icon_url(user))
        })
        .to_owned();

    Ok(embed)
}

pub fn run_subscribe_command(
    db: &PgPool,
    guild_id: &GuildId,
    user: &User,
    _options: &[CommandDataOption],
) -> Result<CreateEmbed, CommandError> {
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
) -> Result<CreateEmbed, CommandError> {
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
) -> Result<CreateEmbed, CommandError> {
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
                        .max_int_value(31)
                        .min_int_value(1)
                        .required(true)
                })
                .create_sub_option(|option| {
                    option
                        .name("month")
                        .description("The month you were born.")
                        .kind(CommandOptionType::Integer)
                        .max_int_value(12)
                        .min_int_value(1)
                        .required(true)
                })
                .create_sub_option(|option| {
                    option
                        .name("year")
                        .description("The year you were born.")
                        .kind(CommandOptionType::Integer)
                        .max_int_value(2100)
                        .min_int_value(1900)
                        .required(true)
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
