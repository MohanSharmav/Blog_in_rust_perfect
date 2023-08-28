use crate::controllers::authentication::session::Password;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::PasswordHash;
use argon2::{Argon2, PasswordHasher};
use sqlx::{Pool, Postgres};

pub async fn password_check(name: String, db: &Pool<Postgres>) -> Result<String, anyhow::Error> {
    let mut login_result = sqlx::query_as::<_, Password>(
        "SELECT  password
            FROM users
            WHERE name = $1
           ",
    )
    .bind(name)
    .fetch_optional(db)
    .await?;

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let dummy_password = "null";
    let dummy_hash = argon2
        .hash_password(dummy_password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    let no_option_result = login_result.get_or_insert_with(|| Password {
        password: dummy_hash,
    });
    let Password { password } = no_option_result;
    let password_has = PasswordHash::new(&*password)?;

    Ok(password_has.to_string())
}
