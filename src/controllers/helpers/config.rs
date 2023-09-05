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
    Ok(db_hashmap.get("db_url").unwrap().into())
}

pub async fn posts_per_page() -> i32 {
    let base_path = std::env::current_dir().unwrap();
    let configuration_directory = base_path.join("configuration");
    let settings = Config::builder()
        .add_source(config::File::from(
            configuration_directory.join("db_configuration.toml"),
        ))
        .build()
        .unwrap();

    let db_hashmap = settings
        .try_deserialize::<HashMap<String, String>>()
        .unwrap();
    let x = db_hashmap.get("SET_POSTS_PER_PAGE").unwrap().to_string();
    let y = x.parse().unwrap();
    y
}
