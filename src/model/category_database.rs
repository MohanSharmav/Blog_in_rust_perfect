use std::arch::asm;
use sqlx::{Error, query_as};
use sqlx::postgres::PgPoolOptions;
use crate::model::database::{categories, posts};
use sqlx::Row;


    pub async fn get_all_categories_database() ->Result<Vec<categories>,Error>
    {
        dotenv::dotenv().expect("Unable to load environment variables from .env file");

        let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

        let mut pool = PgPoolOptions::new()
            .max_connections(100)
            .connect(&db_url)
            .await.expect("Unable to connect to Postgres");

        let  all_categories = sqlx::query_as::<_, categories>("select name,id from categories")

      //  let  rows = sqlx::query_as::<_,categories>("SELECT * FROM categories")
            .fetch_all(&pool)
            .await.expect("Unable to");


        Ok(all_categories)
}


pub async fn category_controller_database_function(category_id:String)->Result<Vec<posts>,Error>
{

println!(" inn database--------   - --{}", category_id);
    let category_id = category_id.parse::<i32>().unwrap();

    // let category_id = category_id as i32;
    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let mut pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await.expect("Unable to connect to Postgres");


    let mut category_posts = sqlx::query_as::<_, posts>("select id,title, description,category_id from posts where category_id=$1")
         .bind(category_id)
        .fetch_all(&pool)
        .await.expect("Unable to get");


    Ok(category_posts)
}

pub async fn create_new_category_database(name: &String, id: &i32) -> Result<(), Error>
{
    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let mut pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await.expect("Unable to connect to Postgres");

    sqlx::query("insert into categories(id,name) values ($1,$2) ")
        .bind(id)
        .bind(name)
        .execute(&pool)
        .await
        .expect("Unable add new category");


    Ok(())
}

pub async fn delete_category_database(to_delete_category: &String) -> Result<(), Error>{

    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let mut pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await.expect("Unable to connect to Postgres");

    let to_delete_category =to_delete_category;

//
// let delete_from_category_table=query_as!("delete from posts where name=$1");
//
//     let delete_from_posts_table =query_as!("delete from posts where name=$1");

//     sqlx::query(r#"delete from posts where name=$1
//
// delete from categories where name=$1"#
// )
   // sqlx::query::join(delete_from_posts_table,delete_from_category_table)
 let   to_delete_category:i32= to_delete_category.parse::<i32>().unwrap();
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

    println!("Successfully deleted");

    Ok(())

}


pub async fn update_category_database(name: &String, category_id: &String) ->Result<(),Error>{
    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let mut pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await.expect("Unable to connect to Postgres");
    // UPDATE Customers
    // SET ContactName = 'Alfred Schmidt', City= 'Frankfurt'
    // WHERE CustomerID = 1;
    let category_id=category_id.parse::<i32>().unwrap();
    sqlx::query("update categories set name=$1 where id=$2")
        .bind(name)
        .bind(category_id)
        .execute(&pool)
        .await
        .expect("Unable toasdasd");
    Ok(())
}



pub async fn category_pagination_controller_database_function(category_id: &String) ->Result<Vec<posts>,Error>
{

    println!(" inn database--------   - --{}", category_id);
    let category_id = category_id.parse::<i32>().unwrap();

    // let category_id = category_id as i32;
    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let mut pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await.expect("Unable to connect to Postgres");


    let mut category_posts = sqlx::query_as::<_, posts>("select id,title, description,category_id from posts where category_id=$1")
        .bind(category_id)
        .fetch_all(&pool)
        .await.expect("Unable to get");


    Ok(category_posts)
}