use sqlx::postgres::PgPoolOptions;
use sqlx::Error;

pub async fn register_new_user_database(user: &String, password: String) -> Result<(), Error> {
    dotenv::dotenv().expect("Unable to load environment variables from .env file");
    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await
        .expect("Unable to connect to Postgres");
    sqlx::query("insert into users(name,password) values ($1,$2)")
        .bind(user)
        .bind(password)
        .execute(&pool)
        .await
        .expect("unable to fetch the user");

    let _user = user.to_string();
    Ok(())
}
