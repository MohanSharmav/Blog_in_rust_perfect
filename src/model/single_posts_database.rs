use crate::model::database::Posts;
use sqlx::postgres::PgPoolOptions;
use sqlx::{ Row};

pub async fn query_single_post(titles: i32) -> Result<Vec<String>, anyhow::Error> {
    dotenv::dotenv()?;
    let db_url = std::env::var("DATABASE_URL")?;
    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await?;

    let mut single_post = Vec::new();
    let rows = sqlx::query("SELECT title,description FROM posts WHERE id=$1")
        .bind(titles)
        .fetch_all(&pool)
        .await?;
    let _x: i32 = 0;
    for row in rows {
        let title: String = row.get("title");
        let description: String = row.get("description");
        let single_post_string = title + " " + &*description;
        single_post.push(single_post_string);
    }
    Ok(single_post)
}
pub async fn query_single_post_in_struct(titles: i32) -> Result<Vec<Posts>, anyhow::Error> {
    dotenv::dotenv()?;
    let db_url = std::env::var("DATABASE_URL")?;
    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await?;

    let single_post = sqlx::query_as::<_, Posts>(
        "select id, title, description, category_id from posts  WHERE id=$1",
    )
    .bind(titles)
    .fetch_all(&pool)
    .await?;
    Ok(single_post)
}
