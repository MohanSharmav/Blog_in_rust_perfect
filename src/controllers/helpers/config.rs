use config::Config;
use std::collections::HashMap;

pub async fn db_config() -> Result<String, anyhow::Error> {
    std::env::current_dir()
        .map(|dir| dir.join("configuration"))
        .map_err(Into::into)
        .and_then(|dir| {
            Config::builder()
                .add_source(config::File::from(dir.join("db_configuration.toml")))
                .build()
                .map_err(anyhow::Error::msg)
        })
        .and_then(|config| config.try_deserialize().map_err(Into::into))
        .and_then(|hashmap: HashMap<String, String>| {
            hashmap
                .get("db_url")
                .map(ToOwned::to_owned)
                .ok_or_else(|| anyhow::anyhow!("db url is missing"))
        })
}
