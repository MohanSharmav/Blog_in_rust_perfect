use sqlx::Error;
use sqlx::postgres::PgPoolOptions;

pub async fn register_new_user_database(user: &String, password: String) -> Result<(),Error>
{
    dotenv::dotenv().expect("Unable to load environment variables from .env file");
println!("🪼{:?}--------------",password);
    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let mut pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await.expect("Unable to connect to Postgres");
  sqlx::query("insert into users(name,password) values ($1,$2)")
        .bind(user)
        .bind(password)
        .execute(&pool)
        .await.expect("unable to fetch the user");

    println!("successfully logged in");
    let user=user.to_string();
 //   let password =password.clone();
    Ok(())
}