pub mod category_repository;
pub mod post_repository;
pub mod user_repository;

pub use category_repository::SqliteCategoryRepository;
pub use post_repository::SqlitePostRepository;
pub use user_repository::SqliteUserRepository;

use crate::error::Result;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::{Pool, Sqlite};
use std::str::FromStr;

/// Connects to a SQLite database, creating the file if it doesn't exist yet,
/// ready to hand to the repository constructors.
pub async fn connect(database_url: &str) -> Result<Pool<Sqlite>> {
    let options = SqliteConnectOptions::from_str(database_url)?.create_if_missing(true);
    Ok(SqlitePoolOptions::new().connect_with(options).await?)
}

/// Applies every migration under `migrations/sqlite` that hasn't run yet.
/// The migration SQL is embedded into the binary at compile time, so this
/// needs no filesystem access at runtime; it's also idempotent, so calling
/// it on every startup (as `blog-server` does) is safe.
pub async fn migrate(pool: &Pool<Sqlite>) -> Result<()> {
    sqlx::migrate!("./migrations/sqlite").run(pool).await?;
    Ok(())
}
