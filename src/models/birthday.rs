use sqlx::{types::chrono::NaiveDateTime, PgPool};
#[derive(Clone)]
pub struct Birthday {
    pub id_birthday: i32,
    guild_id: i64,
    user_id: i64,
    pub date: NaiveDateTime,
    pub create_date: NaiveDateTime,
    pub modify_date: Option<NaiveDateTime>,
}

impl Birthday {
    pub async fn get(
        db: &PgPool,
        guild_id: u64,
        user_id: u64,
    ) -> Result<Option<Birthday>, sqlx::Error> {
        let birthday: Option<Birthday> = sqlx::query_as!(
            Birthday,
            "SELECT id_birthday, guild_id, user_id, date, create_date, modify_date
                FROM birthday
                WHERE guild_id = $1
                AND user_id = $2;",
            (guild_id as i64),
            (user_id as i64),
        )
        .fetch_all(db)
        .await?
        .into_iter()
        .nth(0);

        Ok(birthday)
    }

    pub async fn insert(&mut self, db: &PgPool) -> Result<(), sqlx::Error> {
        let id = sqlx::query!(
            "INSERT INTO birthday 
                (guild_id, user_id, date, create_date)
                VALUES
                ($1, $2, $3, $4)
                RETURNING id_birthday;",
                self.guild_id,
                self.user_id,
                self.date,
                self.create_date,
        )
        .fetch_one(db)
        .await?
        .id_birthday;

        self.id_birthday = id;

        Ok(())
    }

    pub fn guild_id(&self) -> u64 {
        self.guild_id as u64
    }

    pub fn user_id(&self) -> u64 {
        self.user_id as u64
    }
}
