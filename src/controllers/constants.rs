use sqlx::{Pool, Postgres};

#[derive(Clone, Debug)]
pub struct Configuration {
    pub database_connection: Pool<Postgres>,
}
