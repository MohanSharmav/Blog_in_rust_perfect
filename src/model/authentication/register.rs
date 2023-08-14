use sqlx::{Pool, Postgres};

pub async fn register_user(
    user: &str,
    password: String,
    db: &Pool<Postgres>,
) -> Result<(), anyhow::Error> {
    sqlx::query("insert into users(name,password) values ($1,$2)")
        .bind(user)
        .bind(password)
        .execute(db)
        .await?;

    Ok(())
}
