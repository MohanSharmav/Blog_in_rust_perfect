use std::error::Error;
use sqlx::postgres::PgPoolOptions;
use crate::model::database::posts;

//new function for selecting specific post with pointers
pub async fn select_specific_pages_post(start_page: &Option<i32>) ->Result<Vec<posts>,sqlx::Error>
{
    let mut start_page= start_page.unwrap();

// //    let end_posts_count ;
//     let end_posts_count = start_page+3;
//     if(start_page==1)
//     {
//         let end_posts_count = 3;
//     }
//         println!("⭐️{}---{}",start_page,end_posts_count);
//
//
    println!("🐒{:?}",start_page);
    println!("🦄{:?}",start_page+2);
   let mut new_start_page = start_page;
    if(start_page>1)
    {
        new_start_page+=2
    }
    println!("🦆🦆🦆{:?}",&new_start_page);
    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");
    // println!("{}{}", start_page,start_page*3);

    let mut pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await.expect("Unable to connect to Postgres");

    let mut perfect_posts = sqlx::query_as::<_, posts>("select * from posts limit  $1 offset $2")
        // .bind(start_page)
        // .bind(start_page+2)
        .bind(new_start_page+3)
        .bind(new_start_page)
        .fetch_all(&pool)
        .await
        .unwrap();

    println!("🐶{:?}",perfect_posts);
    Ok(perfect_posts)
}