use crate::model::database::{Categories, Posts};
use sqlx::postgres::PgPoolOptions;
pub async fn get_all_categories_database() -> Result<Vec<Categories>, anyhow::Error> {
    dotenv::dotenv()?;
    let db_url = std::env::var("DATABASE_URL")?;
    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await?;

    let all_categories = sqlx::query_as::<_, Categories>("select name,id from categories")
        .fetch_all(&pool)
        .await?;

    Ok(all_categories)
}

pub async fn create_new_category_database(name: &String, id: &i32) -> Result<(), anyhow::Error> {
    dotenv::dotenv()?;
    let db_url = std::env::var("DATABASE_URL")?;
    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await?;

    sqlx::query("insert into categories(id,name) values ($1,$2) ")
        .bind(id)
        .bind(name)
        .execute(&pool)
        .await?;

    Ok(())
}

pub async fn delete_category_database(to_delete_category: &str) -> Result<(), anyhow::Error> {
    dotenv::dotenv()?;
    let db_url = std::env::var("DATABASE_URL")?;
    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await?;
    let to_delete_category: i32 = to_delete_category.parse::<i32>()?;
    sqlx::query("delete from posts where category_id=$1")
        .bind(to_delete_category)
        .execute(&pool)
        .await?;

    sqlx::query("delete from categories where id=$1")
        .bind(to_delete_category)
        .execute(&pool)
        .await?;

    Ok(())
}

pub async fn update_category_database(
    name: &String,
    category_id: &str,
) -> Result<(), anyhow::Error> {
    dotenv::dotenv()?;
    let db_url = std::env::var("DATABASE_URL")?;
    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await?;

    let category_id = category_id.parse::<i32>()?;
    sqlx::query("update categories set name=$1 where id=$2")
        .bind(name)
        .bind(category_id)
        .execute(&pool)
        .await?;
    Ok(())
}

pub async fn category_pagination_controller_database_function(
    category_id: &str,
) -> Result<Vec<Posts>, anyhow::Error> {
    let category_id = category_id.parse::<i32>()?;
    dotenv::dotenv()?;
    let db_url = std::env::var("DATABASE_URL")?;
    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await?;

    let category_posts = sqlx::query_as::<_, Posts>(
        "select id,title, description,category_id from posts where category_id=$1",
    )
    .bind(category_id)
    .fetch_all(&pool)
    .await?;

    Ok(category_posts)
}
