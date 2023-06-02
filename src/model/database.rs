use std::arch::asm;
use actix_web::guard::Post;
use serde::Serialize;
use sqlx::{Error, Pool, Postgres, Row};
use sqlx::postgres::{PgPoolOptions, PgRow};
use serde::Deserialize;
use sqlx::FromRow;

#[derive(Deserialize)]
#[derive(Debug, Clone, PartialEq,Serialize,sqlx::FromRow)]
pub struct categories {
     pub(crate) id:i32,
    pub(crate) name: String,

}

#[derive(Deserialize)]
#[derive(Debug, Clone, PartialEq,Serialize,sqlx::FromRow)]
pub struct posts{
    pub(crate) id: i32,
    pub(crate) title: String,
    pub(crate) description: String,
    pub(crate) category_id: i32,

}

#[derive(Deserialize)]
#[derive(Debug, Clone, PartialEq,Serialize,sqlx::FromRow)]
pub struct update_post{
    pub(crate) current_title: String,
    pub(crate) title: String,
    pub(crate) description: String,
    pub(crate) name: String,
}




pub(crate) async fn get_all_categories() ->Result<Vec<String>, Error>{


    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let mut pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await.expect("Unable to connect to Postgres");


let mut vect=Vec::new();
    let  rows = sqlx::query("SELECT name FROM categories")
        .fetch_all(&pool)
        .await.expect("Unable to");

    for row in rows{
        let names: String=row.get("name");

      //  let original_Array =Foo { name: names.to_string() };

        vect.push(names);

    }


    Ok(vect)
}




pub async fn select_all_from_table() -> Result<Vec<String>,Error> {

    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let mut pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await.expect("Unable to connect to Postgres");

    let mut all_posts = Vec::new();


    let rows = sqlx::query("SELECT title,description FROM posts")
          .fetch_all(&pool)
          .await?;
    for row in rows {
        let title: String = row.get("title");
        let description: String = row.get("description");
       // let all_posts_string=format!(title, description, name);
     //   let all_posts_json = posts { title: title.to_string(), description: description.to_string(), name: name.to_string() };
    }

    let  x:i32= all_posts.len() as i32;
    println!("xxxxxxxx {:?}",x);

//let all_posts_json=serde_json::to_string(&all_posts).expect("noooooo");
    Ok(all_posts)
}

pub async fn select_posts()->Result<Vec<posts>,Error>
{
    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let mut pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await.expect("Unable to connect to Postgres");


    let mut postsing = sqlx::query_as::<_, posts>("select id, title, description, category_id from posts")
        .fetch_all(&pool)
        .await
        .unwrap();

    Ok(postsing)
}
