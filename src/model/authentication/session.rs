use crate::controllers::authentication::session::Password;
use argon2::PasswordHash;
use sqlx::{Pool, Postgres};

pub async fn password_check(name: String, db: &Pool<Postgres>) -> Result<String, anyhow::Error> {
    let login_result = sqlx::query_as::<_, Password>(
        "SELECT  password
            FROM users
            WHERE name = $1
           ",
    )
    .bind(name)
    .fetch_one(db)
    .await?;

    let Password { password } = login_result;
    let password_has = PasswordHash::new(&*password)?;

    Ok(password_has.to_string())
}
