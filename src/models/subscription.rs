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
    pub fn new(
        guild_id: u64,
        user_id: u64,
        birthday_id: i32,
        create_date: NaiveDateTime,
    ) -> Subscription {
        Subscription {
            id_subscription: 0,
            guild_id: guild_id as i64,
            user_id: user_id as i64,
            birthday_id,
            create_date,
            modify_date: None,
        }
    }

    pub async fn get(
        db: &PgPool,
        guild_id: u64,
        user_id: u64,
        birthday_id: i32,
    ) -> Result<Option<Subscription>, sqlx::Error> {
        let subscription: Option<Subscription> = sqlx::query_as!(
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

        Ok(subscription)
    }

    pub async fn get_all_by_guild_and_user(
        db: &PgPool,
        guild_id: u64,
        user_id: u64,
    ) -> Result<Vec<Subscription>, sqlx::Error> {
        let subscriptions: Vec<Subscription> = sqlx::query_as!(
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

        Ok(subscriptions)
    }

    pub async fn get_all_by_birthday_id(
        db: &PgPool,
        birthday_id: i32,
    ) -> Result<Vec<Subscription>, sqlx::Error> {
        let subscriptions: Vec<Subscription> = sqlx::query_as!(
            Subscription,
            "SELECT id_subscription, guild_id, user_id, birthday_id, create_date, modify_date
                FROM subscription
                WHERE birthday_id = $1;",
            birthday_id,
        )
        .fetch_all(db)
        .await?;

        Ok(subscriptions)
    }

    pub async fn insert(&mut self, db: &PgPool) -> Result<(), sqlx::Error> {
        let id = sqlx::query!(
            "INSERT INTO subscription 
                (guild_id, user_id, birthday_id, create_date)
                VALUES
                ($1, $2, $3, $4)
                RETURNING id_subscription;",
            self.guild_id,
            self.user_id,
            self.birthday_id,
            self.create_date,
        )
        .fetch_one(db)
        .await?
        .id_subscription;

        self.id_subscription = id;

        Ok(())
    }

    pub async fn delete(&self, db: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "DELETE FROM subscription WHERE id_subscription = $1",
            self.id_subscription
        )
        .execute(db)
        .await?;

        Ok(())
    }

    pub fn guild_id(&self) -> u64 {
        self.guild_id as u64
    }

    pub fn user_id(&self) -> u64 {
        self.user_id as u64
    }
}
