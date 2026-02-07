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

pub async fn insert_update_log(
    client: &PgPool,
    token_id: &str,
    version: &str,
    files_processed: i32,
    features_added: i32,
    compliance_score: Option<f64>,
    latency: Option<&str>,
    sync_status: Option<&str>,
    raw_payload: &serde_json::Value,
) -> Result<()> {
    client
        .execute(
            "INSERT INTO update_log_v1_7 \
             (token_id, version, files_processed, features_added, \
              compliance_score, latency, sync_status, raw_payload) \
             VALUES ($1,$2,$3,$4,$5,$6,$7,$8)",
            &[
                &token_id,
                &version,
                &files_processed,
                &features_added,
                &compliance_score,
                &latency,
                &sync_status,
                raw_payload,
            ],
        )
        .await?;
    Ok(())
}
