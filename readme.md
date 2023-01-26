# Birthdaybot:

## Technologies:
 - Rust
 - Postgresql
 - Crates: 
   - sqlx
   - serenity
   - poise?
   - chrono
   - serde?
   - tracing

## Features:
 - set birthday
 - remove birthday
 - subscribe birthday
 - unsubscribe birthday

## Commands:

 - `/birthday info`
    - display info about the users birthfay data.
 - `/birthday set <day>`
    - sets the birthday of oneself.
    - `day` the day when the birthday is.
 - `/birthday remove <day>`
    - removes the birthday someone set.
    - `day` the day when the birthday is.
 - `/birthday subscribe <user> <time>`
    - subscribes to some users birthday if found.
    - `user` the user whose birthday should be subscribed to.
    - `time` the time when a message should be send.
 - `/birthday unsubscribe <user>`
    - unsubscribes from someones birthday.
    - `user` the user whose birthday should be unsubscribed from
 - `/birthday subscriptions`
    - shows all subscriptions of a user.
 - `/birthday clear-all`
    - deletes all data the bot has about the user using this command.

## Database:

### Birthday: 

| PK/FK | Name | Type | Nullable | Default | Other |
|-------|------|------|----------|---------|-------|
| PK | id_birthday | int | false | - | A_I |
| UK | guild_id | bigint | false | - | unsigned |
| UK | user_id | bigint | false | - | unsigned |
| | birthday | DateTime | false | - | | 
| | create_date | DateTime | false | - | |
| | modify_date | DateTime | false | - | |

### Subscription:

| PK/FK | Name | Type | Nullable | Default | Other |
|-------|------|------|----------|---------|-------|
| PK | id_subscription | int | false | - | A_I |
| UK | guild_id | bigint | false | - | unsigned |
| UK | user_id | bigint | false | - | unsigned |
| FK | birthday_id | int | false | - | |
| | create_date | DateTime | false | - | |
| | modify_date | DateTime | false | - | |

## Notes:
 - Needs a loop to query over data every some time to then send the birthdays