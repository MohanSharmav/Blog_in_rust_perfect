use crate::model::database::Posts;
use sqlx::postgres::PgPoolOptions;

pub async fn select_specific_pages_post(
    start_page: &Option<i32>,
) -> Result<Vec<Posts>, sqlx::Error> {
    let start_page = start_page.unwrap();

    let mut new_start_page = start_page;

    if start_page > 1 {
        new_start_page += 2
    }
    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await
        .expect("Unable to connect to Postgres");

    let perfect_posts = sqlx::query_as::<_, Posts>("select * from posts limit  $1 offset $2")
        .bind(new_start_page + 3)
        .bind(new_start_page)
        .fetch_all(&pool)
        .await
        .unwrap();

    Ok(perfect_posts)
}

pub async fn select_specific_category_post(
    start_page: &Option<i32>,
    category_input: &str,
) -> Result<Vec<Posts>, sqlx::Error> {
    let start_page = start_page.unwrap();
    let category_id = category_input.parse::<i32>().unwrap();
    let mut new_start_page = start_page;
    if start_page > 1 {
        new_start_page += 2
    }
    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await
        .expect("Unable to connect to Postgres");

    let perfect_posts =
        sqlx::query_as::<_, Posts>("select * from posts where category_id=$3 limit  $1 offset $2")
            .bind(new_start_page + 3)
            .bind(new_start_page)
            .bind(category_id)
            .fetch_all(&pool)
            .await
            .unwrap();

    Ok(perfect_posts)
}
