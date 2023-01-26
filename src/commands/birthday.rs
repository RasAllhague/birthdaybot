use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use serenity::model::prelude::GuildId;
use serenity::model::user::User;
use sqlx::PgPool;

pub fn run_info_command(db: &PgPool, guild_id: &GuildId, user: &User, _options: &[CommandDataOption]) -> String {
    format!("Info command from guild: {}, user: {}", guild_id, user.id)
}

pub fn run_set_command(db: &PgPool, guild_id: &GuildId, user: &User, _options: &[CommandDataOption]) -> String {
    format!("Set command from guild: {}, user: {}", guild_id, user.id)
}

pub fn run_remove_command(db: &PgPool, guild_id: &GuildId, user: &User, _options: &[CommandDataOption]) -> String {
    format!("Remove command from guild: {}, user: {}", guild_id, user.id)
}

pub fn run_subscribe_command(db: &PgPool, guild_id: &GuildId, user: &User, _options: &[CommandDataOption]) -> String {
    format!(
        "Subscribe command from guild: {}, user: {}",
        guild_id, user.id
    )
}

pub fn run_unsubscribe_command(db: &PgPool, guild_id: &GuildId, user: &User, _options: &[CommandDataOption]) -> String {
    format!(
        "Unsubscribe command from guild: {}, user: {}",
        guild_id, user.id
    )
}

pub fn run_clear_command(db: &PgPool, guild_id: &GuildId, user: &User, _options: &[CommandDataOption]) -> String {
    format!("Clear command from guild: {}, user: {}", guild_id, user.id)
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
