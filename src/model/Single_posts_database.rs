use sqlx::{Error, FromRow, Row};
use sqlx::postgres::{PgPoolOptions, PgRow};
use crate::model::database::posts;

pub async fn query_single_post(titles: i32) ->Result<Vec<String>,Error>
{

    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let mut pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await.expect("Unable to connect to Postgres");

    let mut single_post=Vec::new();

    let  rows = sqlx::query("SELECT title,description FROM posts WHERE id=$1")
        .bind(titles)
        .fetch_all(&pool)
        .await.expect("Unable to");
let mut x:i32 =0;

  //  let mut sii:Vec<posts>=Vec::new();

    for row in rows {

       let  title:String= row.get("title");
        let description: String = row.get("description");
        // let name: String = row.get("name");
      //  println!("⭐ ⭐ ⭐ ⭐ ⭐ ⭐ ⭐ ⭐ ⭐{}", title+" "+ &*description +" "+ &*name);
   let single_post_string=title+" "+ &*description ;
        single_post.push(single_post_string);
    }
println!("{:?}",single_post);
//println!("siiii  i i i ⭐ ⭐ ⭐ ⭐ ⭐ ⭐ ⭐ ⭐ {:?}",sii.get(1));
    Ok(single_post)
}