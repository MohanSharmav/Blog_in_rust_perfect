use serde::Deserialize;
use serde::Serialize;
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;

#[derive(Deserialize, Debug, Clone, PartialEq, Serialize, sqlx::FromRow)]
pub struct Categories {
    pub(crate) id: i32,
    pub(crate) name: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Serialize, sqlx::FromRow)]
pub struct Posts {
    pub(crate) id: i32,
    pub(crate) title: String,
    pub(crate) description: String,
    pub(crate) category_id: i32,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Serialize, sqlx::FromRow)]
pub struct UpdatePost {
    pub(crate) current_title: String,
    pub(crate) title: String,
    pub(crate) description: String,
    pub(crate) name: String,
}

pub async fn select_posts() -> Result<Vec<Posts>, anyhow::Error> {
    dotenv::dotenv()?;
    let db_url = std::env::var("DATABASE_URL")?;

    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await?;

    let postsing =
        sqlx::query_as::<_, Posts>("select id, title, description, category_id from posts")
            .fetch_all(&pool)
            .await?;

    Ok(postsing)
}

pub async unsafe fn get_database_connection() -> Result<Pool<Postgres>,anyhow::Error> {
    dotenv::dotenv()?;
   let db_url = std::env::var("DATABASE_URL")?;

let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&*std::env::var("DATABASE_URL")?)
        .await?;
    Ok(pool)
}