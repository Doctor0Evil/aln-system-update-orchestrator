use crate::kafka::Config;
use anyhow::Result;
use rdkafka::{
    config::ClientConfig,
    consumer::{Consumer, StreamConsumer},
};
use tracing::info;

pub type KafkaConsumer = StreamConsumer;

pub async fn build_consumer(cfg: &Config) -> Result<KafkaConsumer> {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", &cfg.bootstrap_servers)
        .set("group.id", &cfg.group_id)
        .set("enable.auto.commit", "true")
        .create()?;
    info!("Kafka consumer initialized with group {}", cfg.group_id);
    Ok(consumer)
}
