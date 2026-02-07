pub mod producer;
pub mod consumer;

use serde::Deserialize;
use std::fs;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub bootstrap_servers: String,
    pub file_update_topic: String,
    pub progress_topic: String,
    pub group_id: String,
}

impl Config {
    pub fn from_file(path: &str) -> anyhow::Result<Self> {
        let raw = fs::read_to_string(path)?;
        Ok(toml::from_str(&raw)?)
    }
}
