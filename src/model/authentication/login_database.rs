use sqlx::{FromRow, Row,Error};
use sqlx::postgres::{PgPoolOptions, PgRow};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct LoginCheck {
    pub(crate) value:i64
}

impl<'r> FromRow<'r, PgRow> for LoginCheck {
    fn from_row(row: &'r PgRow) -> Result<Self,Error> {
        let name = row.try_get("count")?;
        Ok(LoginCheck{ value:name })
    }
}


pub async fn login_database(users: &String, password: String) -> Result<LoginCheck,anyhow::Error > {
    dotenv::dotenv()?;
    let db_url = std::env::var("DATABASE_URL")?;
    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await?;

    let v = sqlx::query_as::<_,LoginCheck>("select count(1) from users where name=$1 AND password=$2")
        .bind(users)
        .bind(password)
        .fetch_one(&pool)
        .await?;

//     for row in v {
//         let title: i64 = row.try_get("count").unwrap();
//         counting_final += title;
//     }
// type(row);
    Ok(v)
}
