use crate::model::database::{Categories, Posts};
use sqlx::postgres::PgPoolOptions;
use sqlx::Error;

pub async fn get_all_categories_database() -> Result<Vec<Categories>, Error> {
    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await
        .expect("Unable to connect to Postgres");

    let all_categories = sqlx::query_as::<_, Categories>("select name,id from categories")
        .fetch_all(&pool)
        .await
        .expect("Unable to");

    Ok(all_categories)
}

pub async fn create_new_category_database(name: &String, id: &i32) -> Result<(), Error> {
    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await
        .expect("Unable to connect to Postgres");

    sqlx::query("insert into categories(id,name) values ($1,$2) ")
        .bind(id)
        .bind(name)
        .execute(&pool)
        .await
        .expect("Unable add new category");

    Ok(())
}

pub async fn delete_category_database(to_delete_category: &str) -> Result<(), Error> {
    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await
        .expect("Unable to connect to Postgres");

    let to_delete_category = to_delete_category;

    let to_delete_category: i32 = to_delete_category.parse::<i32>().unwrap();
    sqlx::query("delete from posts where category_id=$1")
        .bind(to_delete_category)
        .execute(&pool)
        .await
        .expect("Unable to delete post");

    sqlx::query("delete from categories where id=$1")
        .bind(to_delete_category)
        .execute(&pool)
        .await
        .expect("Unable to delete post");

    Ok(())
}

pub async fn update_category_database(name: &String, category_id: &str) -> Result<(), Error> {
    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await
        .expect("Unable to connect to Postgres");
    let category_id = category_id.parse::<i32>().unwrap();
    sqlx::query("update categories set name=$1 where id=$2")
        .bind(name)
        .bind(category_id)
        .execute(&pool)
        .await
        .expect("Unable toasdasd");
    Ok(())
}

pub async fn category_pagination_controller_database_function(
    category_id: &str,
) -> Result<Vec<Posts>, Error> {
    let category_id = category_id.parse::<i32>().unwrap();
    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await
        .expect("Unable to connect to Postgres");

    let category_posts = sqlx::query_as::<_, Posts>(
        "select id,title, description,category_id from posts where category_id=$1",
    )
    .bind(category_id)
    .fetch_all(&pool)
    .await
    .expect("Unable to get");

    Ok(category_posts)
}
