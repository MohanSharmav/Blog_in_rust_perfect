use crate::model::database::Posts;
use sqlx::postgres::PgPoolOptions;

pub async fn select_specific_pages_post(start_page: i32) -> Result<Vec<Posts>, anyhow::Error> {
    let start_page = start_page;
    let mut new_start_page = start_page;
    if start_page > 1 {
        new_start_page += 2
    }
    dotenv::dotenv()?;
    let db_url = std::env::var("DATABASE_URL")?;
    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await?;

    let perfect_posts = sqlx::query_as::<_, Posts>("select * from posts limit  $1 offset $2")
        .bind(new_start_page + 3)
        .bind(new_start_page)
        .fetch_all(&pool)
        .await?;

    Ok(perfect_posts)
}
