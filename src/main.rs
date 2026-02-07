use anyhow::Result;
use tracing_subscriber::EnvFilter;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::sync::Arc;
use tokio::sync::oneshot;
use tracing::{info, error};

use aln_system_update_orchestrator::{
    db,
    kafka,
    orchestrator::Orchestrator,
    opa,
};

async fn health() -> impl Responder {
    HttpResponse::Ok()
        .content_type("application/json")
        .body(r#"{"status":"ok","service":"aln-system-update-orchestrator"}"#)
}

async fn readiness() -> impl Responder {
    HttpResponse::Ok()
        .content_type("application/json")
        .body(r#"{"status":"ready"}"#)
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let kafka_cfg = kafka::Config::from_file("config/kafka.toml")?;
    let pg_cfg = db::postgres::Config::from_file("config/postgres.toml")?;
    let redis_cfg = db::redis::Config::from_file("config/redis.toml")?;
    let opa_client = opa::Client::new(
        std::env::var("OPA_ADDR").unwrap_or_else(|_| "http://127.0.0.1:8181".into()),
    );

    let pg_pool = db::postgres::connect(pg_cfg).await?;
    let redis_client = db::redis::connect(redis_cfg).await?;

    let kafka_producer = kafka::producer::build_producer(&kafka_cfg).await?;
    let kafka_consumer = kafka::consumer::build_consumer(&kafka_cfg).await?;

    let orchestrator = Arc::new(Orchestrator::new(
        kafka_cfg.clone(),
        kafka_producer,
        kafka_consumer,
        pg_pool,
        redis_client,
        opa_client,
    ));

    let (tx, rx) = oneshot::channel::<()>();
    let orchestrator_clone = orchestrator.clone();

    tokio::spawn(async move {
        if let Err(e) = orchestrator_clone.run().await {
            error!("Orchestrator error: {:?}", e);
        }
        let _ = tx.send(());
    });

    info!("Starting HTTP health server on 0.0.0.0:8080");
    HttpServer::new(move || {
        App::new()
            .route("/health", web::get().to(health))
            .route("/ready", web::get().to(readiness))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await?;

    let _ = rx.await;
    Ok(())
}
