use crate::controllers::authentication::session::PasswordStruct;
use sqlx::{Pool, Postgres};
use std::option::Option;
use serde::Deserialize;

pub async fn password_check(
    name: &String,
    db: &Pool<Postgres>,
) -> Result<Option<PasswordStruct>, anyhow::Error> {
    // select password of the user and store it into option
    // if user exists, then Option<"argon_sting">
    // if no user exists, then Option<None>
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

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct User {
    pub(crate) username: String,
    pub(crate) password: String,
}
