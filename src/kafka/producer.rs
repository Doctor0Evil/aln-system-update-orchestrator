use crate::kafka::Config;
use anyhow::Result;
use rdkafka::{
    config::ClientConfig,
    producer::{FutureProducer, FutureRecord},
};
use serde_json::json;
use std::time::Duration;
use tracing::info;

pub type KafkaProducer = FutureProducer;

pub async fn build_producer(cfg: &Config) -> Result<KafkaProducer> {
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", &cfg.bootstrap_servers)
        .create()?;
    Ok(producer)
}

pub async fn publish_file_update(
    producer: &KafkaProducer,
    topic: &str,
    version: &str,
) -> Result<()> {
    let payload = json!({
        "version": version,
        "event": "file_update",
    })
    .to_string();

    producer
        .send(
            FutureRecord::to(topic).payload(&payload),
            Duration::from_secs(0),
        )
        .await?;
    info!("Published file_update event for version {}", version);
    Ok(())
}

pub async fn publish_progress(
    producer: &KafkaProducer,
    topic: &str,
    version: &str,
    files_processed: i32,
    features_added: i32,
) -> Result<()> {
    let payload = json!({
        "version": version,
        "event": "update_progress",
        "files_processed": files_processed,
        "features_added": features_added,
    })
    .to_string();

    producer
        .send(
            FutureRecord::to(topic).payload(&payload),
            Duration::from_secs(0),
        )
        .await?;
    info!("Published update_progress event for version {}", version);
    Ok(())
}
