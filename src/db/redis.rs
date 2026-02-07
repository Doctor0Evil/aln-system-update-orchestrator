use anyhow::Result;
use serde::Deserialize;
use serde_json::Value;
use std::fs;
use rustis::{client::Client, Result as RedisResult}; // [web:21]

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub address: String,
    pub state_prefix: String,
}

pub type RedisClient = Client;

impl Config {
    pub fn from_file(path: &str) -> Result<Self> {
        let raw = fs::read_to_string(path)?;
        Ok(toml::from_str(&raw)?)
    }
}

pub async fn connect(cfg: Config) -> Result<RedisClient> {
    let client = Client::connect(cfg.address).await?;
    Ok(client)
}

pub async fn save_state(
    client: &RedisClient,
    token_id: &str,
    version: &str,
    features: &[String],
) -> Result<()> {
    let key = format!("aln_update_state_1.0.1.7:{}", token_id);
    let value = Value::from(features.to_vec()).to_string();
    let _: RedisResult<()> = client.set(key, value).await?;
    Ok(())
}
