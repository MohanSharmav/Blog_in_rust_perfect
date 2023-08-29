use std::collections::HashMap;
use config::Config;
use serde::Deserializer;

pub async fn db_config()-> Result<String,anyhow::Error>{
    let base_path = std::env::current_dir()
        .expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("configuration");
    let settings = Config::builder()
        .add_source(config::File::from(
            configuration_directory.join("db_conf.toml"),
        )).build()?;

    let db_hashmap=settings.try_deserialize::<HashMap<String, String>>()?;
    let default_url="postgres://mohanvenkatesh:Msvmsd183!@localhost:5432/3_dummy".to_string();
    let y=db_hashmap.get("db_url").get_or_insert(&default_url).to_string();

    Ok(y.clone())
}