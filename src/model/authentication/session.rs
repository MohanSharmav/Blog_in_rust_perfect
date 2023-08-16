use crate::model::structs::LoginCheck;
use sqlx::{Pool, Postgres};

pub async fn login_database(
    users: &String,
    password: String,
    db: &Pool<Postgres>,
) -> Result<LoginCheck, anyhow::Error> {
    let v =
        sqlx::query_as::<_, LoginCheck>("select count(1) from users where name=$1 AND password=$2")
            .bind(users)
            .bind(password)
            .fetch_one(db)
            .await?;

    Ok(v)
}
