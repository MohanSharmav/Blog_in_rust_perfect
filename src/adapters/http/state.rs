use crate::adapters::crypto::MagicCryptCipher;
use anyhow::Result;

// The concrete repository types are picked at compile time by exactly one of
// the `postgres`/`sqlite` features (enforced in `main.rs`), never by a
// runtime enum — swapping backends means rebuilding with a different
// feature, not branching at runtime.
#[cfg(feature = "postgres")]
use blog_storage::postgres::{
    PgCategoryRepository as Categories, PgPostRepository as Posts, PgUserRepository as Users,
};
#[cfg(feature = "sqlite")]
use blog_storage::sqlite::{
    SqliteCategoryRepository as Categories, SqlitePostRepository as Posts,
    SqliteUserRepository as Users,
};

/// The composed application state: one concrete adapter per port, wired once
/// at the composition root (`main.rs`) and shared across requests.
#[derive(Clone)]
pub struct AppState {
    pub posts: Posts,
    pub categories: Categories,
    pub users: Users,
    pub cipher: MagicCryptCipher,
}

impl AppState {
    /// Connects to `database_url` with whichever backend was compiled in and
    /// builds the composed state.
    pub async fn connect(database_url: &str, magic_key: &str) -> Result<Self> {
        #[cfg(feature = "postgres")]
        let pool = blog_storage::postgres::connect(database_url).await?;
        #[cfg(feature = "sqlite")]
        let pool = blog_storage::sqlite::connect(database_url).await?;

        #[cfg(feature = "postgres")]
        blog_storage::postgres::migrate(&pool).await?;
        #[cfg(feature = "sqlite")]
        blog_storage::sqlite::migrate(&pool).await?;

        Ok(Self {
            posts: Posts::new(pool.clone()),
            categories: Categories::new(pool.clone()),
            users: Users::new(pool),
            cipher: MagicCryptCipher::new(magic_key),
        })
    }
}
