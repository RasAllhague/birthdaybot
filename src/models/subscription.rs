use sqlx::{types::chrono::NaiveDateTime, PgPool};

pub struct Subscription {
    pub id_subscription: i32,
    guild_id: i64,
    user_id: i64,
    pub birthday_id: i32,
    pub create_date: NaiveDateTime,
    pub modify_date: Option<NaiveDateTime>,
}

pub struct SendNotification {
    pub id_send_notification: i32,
    pub subscription_id: i32,
    pub current_year: i32,
    pub create_date: NaiveDateTime,
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
        year: i32,
    ) -> Result<Vec<Subscription>, sqlx::Error> {
        let subscriptions: Vec<Subscription> = sqlx::query_as!(
            Subscription,
            "SELECT s.id_subscription, s.guild_id, s.user_id, s.birthday_id, s.create_date, s.modify_date
                FROM subscription AS s
                LEFT JOIN send_notifications AS sn
                ON s.id_subscription = sn.subscription_id AND sn.current_year = $1
                WHERE s.birthday_id = $2
                AND sn.id_send_notification is NULL;",
            year,
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
    
    pub fn user_id(&self) -> u64 {
        self.user_id as u64
    }
}

impl SendNotification {
    pub fn new(subscription_id: i32, current_year: i32, create_date: NaiveDateTime) -> Self {
        Self {
            id_send_notification: 0,
            subscription_id: subscription_id,
            current_year: current_year,
            create_date,
        }
    }

    pub async fn insert(&mut self, db: &PgPool) -> Result<(), sqlx::Error> {
        let id = sqlx::query!(
            "INSERT INTO send_notifications 
                (subscription_id, current_year, create_date)
                VALUES
                ($1, $2, $3)
                RETURNING id_send_notification;",
            self.subscription_id,
            self.current_year,
            self.create_date,
        )
        .fetch_one(db)
        .await?
        .id_send_notification;

        self.id_send_notification = id;

        Ok(())
    }
}