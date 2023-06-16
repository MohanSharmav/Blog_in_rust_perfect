use serde::Deserialize;
use sqlx::postgres::PgRow;
use sqlx::{Error, FromRow, Pool, Postgres, Row};

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct LoginCheck {
    pub(crate) value: i64,
}

impl<'r> FromRow<'r, PgRow> for LoginCheck {
    fn from_row(row: &'r PgRow) -> Result<Self, Error> {
        let name = row.try_get("count")?;
        Ok(LoginCheck { value: name })
    }
}

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
