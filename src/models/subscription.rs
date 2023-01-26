use sqlx::{types::chrono::NaiveDateTime, PgPool};

pub struct Subscription {
    pub id_subscription: i32,
    guild_id: i64,
    user_id: i64,
    pub birthday_id: i32,
    pub create_date: NaiveDateTime,
    pub modify_date: Option<NaiveDateTime>,
}

impl Subscription {
    pub async fn get(
        db: &PgPool,
        guild_id: u64,
        user_id: u64,
        birthday_id: i32,
    ) -> Result<Option<Subscription>, sqlx::Error> {
        let birthday: Option<Subscription> = sqlx::query_as!(
            Subscription,
            "SELECT id_subscription, guild_id, user_id, birthday_id, create_date, modify_date
                FROM subscription
                WHERE guild_id = $1
                AND user_id = $2
                AND birthday_id = $3;",
            (guild_id as i64),
            (user_id as i64),
            birthday_id
        )
        .fetch_all(db)
        .await?
        .into_iter()
        .nth(0);

        Ok(birthday)
    }

    pub async fn get_all(
        db: &PgPool,
        guild_id: u64,
        user_id: u64,
    ) -> Result<Vec<Subscription>, sqlx::Error> {
        let birthday: Vec<Subscription> = sqlx::query_as!(
            Subscription,
            "SELECT id_subscription, guild_id, user_id, birthday_id, create_date, modify_date
                FROM subscription
                WHERE guild_id = $1
                AND user_id = $2;",
            (guild_id as i64),
            (user_id as i64)
        )
        .fetch_all(db)
        .await?;

        Ok(birthday)
    }

    pub fn guild_id(&self) -> u64 {
        self.guild_id as u64
    }

    pub fn user_id(&self) -> u64 {
        self.user_id as u64
    }
}
