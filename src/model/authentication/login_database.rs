use sqlx::{Error, Row};
use sqlx::postgres::PgPoolOptions;
use crate::controller::authentication::login::user;

pub async fn login_database(users: &String, password: String) -> (i64, )
{
    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    println!("🦄---password in database is{:?}",password);
    let mut pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await.expect("Unable to connect to Postgres");

  let v: (i64,)=  sqlx::query_as("select count(1) from users where name=$1 AND password=$2")
        .bind(users)
        .bind(password)
        .fetch_one(&pool)
        .await.expect("unable to fetch the user");
    //
    // let mut v = sqlx::query_as::<_, user>("select name,password from users where name=$1")
    //     .bind(users)
    //     .fetch_one(&pool)
    //     .await.expect("unable to fetch the password");
    println!("password from db is 🐯{:?}", v);

    v
}