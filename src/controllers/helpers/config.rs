pub async fn db_config() -> Result<String, anyhow::Error> {
    let db_url = "postgres://mohanvenkatesh:Msvmsd183!@localhost:5432/3_dummy".to_string();
    Ok(db_url)
}