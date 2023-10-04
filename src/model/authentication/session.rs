use crate::controllers::authentication::session::PasswordStruct;
use serde::Deserialize;
use sqlx::{Pool, Postgres};
use std::option::Option;

pub async fn password_check(
    name: &String,
    db: &Pool<Postgres>,
) -> Result<Option<PasswordStruct>, anyhow::Error> {
    // select password of the user and store it into option
    // if user exists, then Option<"argon_sting">
    // if no user exists, then Option<None>
    sqlx::query_as::<_, PasswordStruct>(
        "SELECT  password
            FROM users
            WHERE name = $1
           ",
    )
    .bind(name)
    .fetch_optional(db)
    .await
    .map_err(Into::into)
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct User {
    pub(crate) username: String,
    pub(crate) password: String,
}
