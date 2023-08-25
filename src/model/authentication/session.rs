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
    let parsed_hash = PasswordHash::new(&*password)?;
    // let parsed_hash = password_check(username.clone(), db)
    //     .await
    //     .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(parsed_hash.to_string())
}
