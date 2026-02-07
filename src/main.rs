use anyhow::Result;
use tracing_subscriber::EnvFilter;

use aln_system_update_orchestrator::{
    db,
    kafka,
    orchestrator::Orchestrator,
    opa,
};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let kafka_cfg = kafka::Config::from_file("config/kafka.toml")?;
    let pg_cfg = db::postgres::Config::from_file("config/postgres.toml")?;
    let redis_cfg = db::redis::Config::from_file("config/redis.toml")?;
    let opa_client = opa::Client::new(std::env::var("OPA_ADDR").unwrap_or_else(|_| "http://127.0.0.1:8181".into()));

    let pg_pool = db::postgres::connect(pg_cfg).await?;
    let redis_client = db::redis::connect(redis_cfg).await?;

    let kafka_producer = kafka::producer::build_producer(&kafka_cfg).await?;
    let kafka_consumer = kafka::consumer::build_consumer(&kafka_cfg).await?;

    let orchestrator = Orchestrator::new(
        kafka_cfg,
        kafka_producer,
        kafka_consumer,
        pg_pool,
        redis_client,
        opa_client,
    );

    orchestrator.run().await?;

    Ok(())
}
