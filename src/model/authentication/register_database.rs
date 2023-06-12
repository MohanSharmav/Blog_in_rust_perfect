use sqlx::postgres::PgPoolOptions;

pub async fn register_new_user_database(user: &str, password: String) -> Result<(), anyhow::Error> {
    dotenv::dotenv()?;
    let db_url = std::env::var("DATABASE_URL")?;
    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await?;

    sqlx::query("insert into users(name,password) values ($1,$2)")
        .bind(user)
        .bind(password)
        .execute(&pool)
        .await?;

    let _user = user.to_string();
    Ok(())
}
