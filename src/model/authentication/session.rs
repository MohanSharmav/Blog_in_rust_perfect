use crate::model::structs::{LoginCheck, Password};
use argon2::PasswordHash;
use sqlx::{Pool, Postgres};

pub async fn login_database(
    users: &String,
    password: String,
    db: &Pool<Postgres>,
) -> Result<LoginCheck, anyhow::Error> {
    let login_result =
        sqlx::query_as::<_, LoginCheck>("select count(1) from users where name=$1 AND password=$2")
            .bind(users)
            .bind(password)
            .fetch_one(db)
            .await?;

    Ok(login_result)
}

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
    // let Password{password} =login_result.password.to_string();
    //
    // let x: &GetId = &post_id[0];
    // let GetId { id } = x;

    let Password { password } = login_result;
    let parsed_hash = PasswordHash::new(&*password)?;
    // let parsed_hash = password_check(username.clone(), db)
    //     .await
    //     .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(parsed_hash.to_string())
}
