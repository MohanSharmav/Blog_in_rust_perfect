#[derive(serde::Deserialize, Clone)]
pub struct Config {
    pub(crate) magic_key: String,
}
