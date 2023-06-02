use std::fmt::Error;
use sqlx::postgres::PgPoolOptions;
use crate::controller::posts_controller::update_post_helper;
use crate::model::database::posts;

pub async fn create_new_post_database(id: &i32, title: &String, description: &String, category_id: &i32) -> Result<(),Error> {
    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let mut pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await.expect("Unable to connect to Postgres");
    sqlx::query("insert into posts(id,title,description,category_id) values ($1,$2,$3,$4)")
        .bind(id)
        .bind(title)
        .bind(description)
        .bind(category_id)
        .execute(&pool)
        .await
        .expect("Unable toasdasd");
    Ok(())

}

pub async fn  delete_post_database(to_delete: String) ->Result<(),Error>
{
    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let mut pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await.expect("Unable to connect to Postgres");

    let to_delete =to_delete.parse::<i32>().unwrap();;

    sqlx::query("delete from posts where id=$1")
        .bind(to_delete)
        .execute(&pool)
        .await
        .expect("Unable toasdasd");
println!("Successfully deleted");
    Ok(())
}

pub async fn update_post_database(title: &String, description: &String, id: &&i32, category_id: &&i32) ->Result<(),Error>{
    dotenv::dotenv().expect("Unable to load environment variables from .env file");
println!("🦋🦋🦋🦋🦋Updating database");
    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let mut pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await.expect("Unable to connect to Postgres");
    // UPDATE Customers
    // SET ContactName = 'Alfred Schmidt', City= 'Frankfurt'
    // WHERE CustomerID = 1;
    sqlx::query("update posts set title=$1 ,description=$2, category_id=$3 where id=$4")
        .bind(title)
        .bind(description)
        .bind(category_id)
        .bind(id)
        .execute(&pool)
        .await
        .expect("Unable toasdasd");
    Ok(())
}