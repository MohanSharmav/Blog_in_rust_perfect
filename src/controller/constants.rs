use magic_crypt::MagicCrypt256;
use sqlx::{Pool, Postgres};

#[derive(Clone, Debug)]
pub struct ConfigurationConstants {
    pub(crate) magic_key: MagicCrypt256,
    pub database_connection: Pool<Postgres>,
}
