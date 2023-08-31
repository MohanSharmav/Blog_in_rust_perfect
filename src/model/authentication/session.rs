use crate::controllers::authentication::session::PasswordStruct;
use sqlx::{Pool, Postgres};
use std::option::Option;

pub async fn password_check(
    name: String,
    db: &Pool<Postgres>,
) -> Result<Option<PasswordStruct>, anyhow::Error> {
    let login_result = sqlx::query_as::<_, PasswordStruct>(
        "SELECT  password
            FROM users
            WHERE name = $1
           ",
    )
    .bind(name)
    .fetch_optional(db)
    .await?;

    Ok::<std::option::Option<PasswordStruct>, anyhow::Error>(login_result)
}
