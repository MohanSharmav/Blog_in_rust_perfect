use crate::error::Result;
use crate::ports::UserRepository;
use sqlx::{Pool, Sqlite};

/// SQLite-backed adapter implementing the [`UserRepository`] port.
#[derive(Clone)]
pub struct SqliteUserRepository {
    pool: Pool<Sqlite>,
}

impl SqliteUserRepository {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }
}

impl UserRepository for SqliteUserRepository {
    async fn register(&self, username: &str, password_hash: String) -> Result<()> {
        sqlx::query("insert into users(name,password) values (?1,?2)")
            .bind(username)
            .bind(password_hash)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn credentials_match(&self, username: &str, password_hash: String) -> Result<bool> {
        let matches: i64 =
            sqlx::query_scalar("select count(1) from users where name=?1 AND password=?2")
                .bind(username)
                .bind(password_hash)
                .fetch_one(&self.pool)
                .await?;
        Ok(matches > 0)
    }
}
