pub async fn register_new_user_database(
    user: &str,
    password: String,
    db: &sqlx::PgPool,
) -> Result<(), anyhow::Error> {
    sqlx::query("insert into users(name,password) values ($1,$2)")
        .bind(user)
        .bind(password)
        .execute(db)
        .await?;

    Ok(())
}
