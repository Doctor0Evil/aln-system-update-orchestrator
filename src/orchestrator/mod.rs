mod steps;

use crate::{
    aln::AlnUpdatePlan,
    db::{postgres::PgPool, redis::RedisClient},
    kafka::{Config as KafkaConfig, consumer::KafkaConsumer, producer::KafkaProducer},
    opa::Client as OpaClient,
};
use anyhow::Result;
use tracing::{info, error};

pub struct Orchestrator {
    kafka_cfg: KafkaConfig,
    producer: KafkaProducer,
    consumer: KafkaConsumer,
    pg_pool: PgPool,
    redis: RedisClient,
    opa: OpaClient,
}

impl Orchestrator {
    pub fn new(
        kafka_cfg: KafkaConfig,
        producer: KafkaProducer,
        consumer: KafkaConsumer,
        pg_pool: PgPool,
        redis: RedisClient,
        opa: OpaClient,
    ) -> Self {
        Self { kafka_cfg, producer, consumer, pg_pool, redis, opa }
    }

    pub async fn run(&self) -> Result<()> {
        info!("Loading ALN update plan...");
        let plan = AlnUpdatePlan::from_file("aln/system_update_integration_v1.7.aln")?;

        info!("Validating plan with OPA...");
        steps::validate_with_opa(&self.opa, &plan).await?;

        info!("Processing files, syncing DBs, and publishing Kafka events...");
        steps::process_files_and_sync(self, &plan).await?;

        info!("Update pipeline completed successfully.");
        Ok(())
    }
}
