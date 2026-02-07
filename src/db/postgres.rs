use anyhow::Result;
use serde::Deserialize;
use std::fs;
use tokio_postgres::{Client, NoTls}; // [web:20][web:24]
use serde_json::Value;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub connection_string: String,
    pub aln_update_table: String,
    pub update_log_table: String,
}

pub type PgPool = Client;

impl Config {
    pub fn from_file(path: &str) -> Result<Self> {
        let raw = fs::read_to_string(path)?;
        Ok(toml::from_str(&raw)?)
    }
}

pub async fn connect(cfg: Config) -> Result<PgPool> {
    let (client, connection) = tokio_postgres::connect(&cfg.connection_string, NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("PostgreSQL connection error: {}", e);
        }
    });
    Ok(client)
}

pub async fn insert_update_record(
    client: &PgPool,
    version: &str,
    features: &[String],
) -> Result<()> {
    let data = Value::from(features.to_vec());
    client
        .execute(
            "INSERT INTO aln_update_data (version, data) VALUES ($1, $2)",
            &[&version, &data],
        )
        .await?;
    Ok(())
}
