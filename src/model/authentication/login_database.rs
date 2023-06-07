use sqlx::postgres::PgPoolOptions;

pub async fn login_database(users: &String, password: String) -> (i64,) {
    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await
        .expect("Unable to connect to Postgres");

    let v: (i64,) = sqlx::query_as("select count(1) from users where name=$1 AND password=$2")
        .bind(users)
        .bind(password)
        .fetch_one(&pool)
        .await
        .expect("unable to fetch the user");

    v
}
