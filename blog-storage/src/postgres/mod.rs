pub mod category_repository;
pub mod post_repository;
pub mod user_repository;

pub use category_repository::PgCategoryRepository;
pub use post_repository::PgPostRepository;
pub use user_repository::PgUserRepository;

use crate::error::Result;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

/// Connects to a Postgres database, ready to hand to the repository constructors.
pub async fn connect(database_url: &str) -> Result<Pool<Postgres>> {
    Ok(PgPoolOptions::new()
        .max_connections(100)
        .connect(database_url)
        .await?)
}

/// Applies every migration under `migrations/postgres` that hasn't run yet.
/// The migration SQL is embedded into the binary at compile time, so this
/// needs no filesystem access at runtime; it's also idempotent, so calling
/// it on every startup (as `blog-server` does) is safe.
pub async fn migrate(pool: &Pool<Postgres>) -> Result<()> {
    sqlx::migrate!("./migrations/postgres").run(pool).await?;
    Ok(())
}
