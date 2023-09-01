use config::Config;
use std::collections::HashMap;

pub async fn db_config() -> Result<String, anyhow::Error> {
    let base_path = std::env::current_dir()?;
    let configuration_directory = base_path.join("configuration");
    let settings = Config::builder()
        .add_source(config::File::from(
            configuration_directory.join("db_configuration.toml"),
        ))
        .build()?;

    let db_hashmap = settings.try_deserialize::<HashMap<String, String>>()?;
    let db_url = db_hashmap.get("db_url").unwrap().to_string();

    Ok(db_url)
}
