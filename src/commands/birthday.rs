use std::collections::HashMap;

use serenity::builder::{CreateApplicationCommand, CreateEmbed};
use serenity::futures::future::join_all;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use serenity::model::prelude::GuildId;
use serenity::model::user::User;
use serenity::prelude::Context;
use sqlx::types::chrono::Utc;
use sqlx::PgPool;

use crate::models::birthday::Birthday;
use crate::models::subscription::Subscription;
use crate::utils;

use super::parser::{DateInputParser, UserInputParser};
use super::CommandError;

pub async fn run_info_command(
    db: &PgPool,
    ctx: &Context,
    guild_id: &GuildId,
    user: &User,
) -> Result<CreateEmbed, CommandError> {
    if let Some(bday) = Birthday::get(db, guild_id.0, user.id.0)
        .await
        .map_err(|x| CommandError::Db(x))?
    {
        let subscriptions = Subscription::get_all(db, guild_id.0, user.id.0)
            .await
            .map_err(|x| CommandError::Db(x))?;

        let fields = subscriptions
            .iter()
            .map(|s| async { gen_embed_field(db, guild_id.0, &ctx, s).await });

        let fields: Result<Vec<(String, String, bool)>, CommandError> =
            join_all(fields).await.into_iter().collect();

        let embed = CreateEmbed(HashMap::new())
            .title("Birthday:")
            .description(format!("{}", bday.date.date()))
            .author(|author| {
                author
                    .name(user.name.clone())
                    .icon_url(utils::get_icon_url(user))
            })
            .field("Subscriptions:", "", false)
            .fields(fields?)
            .to_owned();

        return Ok(embed);
    }

    let embed = CreateEmbed(HashMap::new())
        .title("Birthday:")
        .description("You have not registered your birthday yet.")
        .author(|author| {
            author
                .name(user.name.clone())
                .icon_url(utils::get_icon_url(user))
        })
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

pub async fn run_subscribe_command(
    db: &PgPool,
    guild_id: &GuildId,
    user: &User,
    options: &[CommandDataOption],
) -> Result<CreateEmbed, CommandError> {
    let user_to_subcribe_to = UserInputParser
        .parse(options, 0)
        .map_err(|x| CommandError::Parser(x))?;

    if let Some(birthday) = Birthday::get(db, guild_id.0, user_to_subcribe_to.id.0)
        .await
        .map_err(|x| CommandError::Db(x))?
    {
        if let None = Subscription::get(db, guild_id.0, user.id.0, birthday.id_birthday)
            .await
            .map_err(|x| CommandError::Db(x))?
        {
            let mut subscription = Subscription::new(
                guild_id.0,
                user.id.0,
                birthday.id_birthday,
                Utc::now().naive_utc(),
            );
            subscription
                .insert(db)
                .await
                .map_err(|x| CommandError::Db(x))?;

            let embed = CreateEmbed(HashMap::new())
                .title("Birthday Subscription:")
                .description(format!(
                    "You are now subcribed to the birthday of <@{}>.",
                    user_to_subcribe_to.id
                ))
                .author(|author| {
                    author
                        .name(user.name.clone())
                        .icon_url(utils::get_icon_url(user))
                })
                .to_owned();

            return Ok(embed);
        }

        let embed = CreateEmbed(HashMap::new())
            .title("Birthday Subscription:")
            .description("You are already subscribed to this persons birthday.")
            .author(|author| {
                author
                    .name(user.name.clone())
                    .icon_url(utils::get_icon_url(user))
            })
            .to_owned();

        return Ok(embed);
    }

    let embed = CreateEmbed(HashMap::new())
        .title("Birthday Subscription:")
        .description("The targeted user does not provide a birthday.")
        .author(|author| {
            author
                .name(user.name.clone())
                .icon_url(utils::get_icon_url(user))
        })
        .to_owned();

    return Ok(embed);
}

pub async fn run_unsubscribe_command(
    db: &PgPool,
    guild_id: &GuildId,
    user: &User,
    options: &[CommandDataOption],
) -> Result<CreateEmbed, CommandError> {
    let user_to_subcribe_to = UserInputParser
        .parse(options, 0)
        .map_err(|x| CommandError::Parser(x))?;

    if let Some(birthday) = Birthday::get(db, guild_id.0, user_to_subcribe_to.id.0)
        .await
        .map_err(|x| CommandError::Db(x))?
    {
        if let Some(subscription) =
            Subscription::get(db, guild_id.0, user.id.0, birthday.id_birthday)
                .await
                .map_err(|x| CommandError::Db(x))?
        {
            subscription
                .delete(db)
                .await
                .map_err(|x| CommandError::Db(x))?;

            let embed = CreateEmbed(HashMap::new())
                .title("Birthday Subscription:")
                .description("Your subscriptions to this birthday has been deleted.")
                .author(|author| {
                    author
                        .name(user.name.clone())
                        .icon_url(utils::get_icon_url(user))
                })
                .to_owned();

            return Ok(embed);
        }

        let embed = CreateEmbed(HashMap::new())
            .title("Birthday Subscription:")
            .description("You have no subscription for this user.")
            .author(|author| {
                author
                    .name(user.name.clone())
                    .icon_url(utils::get_icon_url(user))
            })
            .to_owned();

        return Ok(embed);
    }

    let embed = CreateEmbed(HashMap::new())
        .title("Birthday Subscription:")
        .description("The user has no birthday set up.")
        .author(|author| {
            author
                .name(user.name.clone())
                .icon_url(utils::get_icon_url(user))
        })
        .to_owned();

    Ok(embed)
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    build_info_command(command);
    build_set_command(command);
    build_remove_command(command);
    build_subscribe_command(command);
    build_unsubscribe_command(command)
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
                .description("Removes all data of birthdays.")
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
                .description("Subscribes to the birthday of another user.")
                .kind(CommandOptionType::SubCommand)
                .create_sub_option(|option| {
                    option
                        .name("user")
                        .description("The user you want to subscribe to.")
                        .kind(CommandOptionType::User)
                        .required(true)
                })
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
                .description("Unsubscribes from a user.")
                .kind(CommandOptionType::SubCommand)
                .create_sub_option(|option| {
                    option
                        .name("user")
                        .description("The user you want to unsubscribe from.")
                        .kind(CommandOptionType::User)
                        .required(true)
                })
        })
}

async fn gen_embed_field(
    db: &sqlx::Pool<sqlx::Postgres>,
    guild_id: u64,
    ctx: &Context,
    subscription: &Subscription,
) -> Result<(String, String, bool), CommandError> {
    let birthday = Birthday::get_by_id(db, subscription.birthday_id)
        .await
        .map_err(|x| CommandError::Db(x))?
        .expect("Birthday should not be delete before subscription.");

    match ctx.http.get_member(guild_id, birthday.user_id()).await {
        Ok(m) => Ok((
            m.display_name().to_string(),
            birthday.date.date().to_string(),
            false,
        )),
        Err(_) => Ok((
            format!("<@{}>:", birthday.user_id()),
            birthday.date.date().to_string(),
            false,
        )),
    }
}
