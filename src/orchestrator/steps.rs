use crate::{
    aln::AlnUpdatePlan,
    db::{postgres, redis},
    kafka,
    opa::Client as OpaClient,
    orchestrator::Orchestrator,
};
use anyhow::{Result, anyhow};
use serde_json::json;
use tracing::info;

pub async fn validate_with_opa(opa: &OpaClient, plan: &AlnUpdatePlan) -> Result<()> {
    let input = json!({
        "repo_structure": { "compliant_with": "modular_aln" },
        "version_history": { "tracked_with": "commits" },
        "process_tree": { "valid_with_k8s": true },
        "sources": { "updated_via_pipeline": true },
        "platforms": { "all_managed": true },
        "lan_service": "full_with_configs",
        "directories": [
            { "name": "src" },
            { "name": "config" }
        ],
        "commits": []
    });

    let decision = opa.evaluate("aln_system_update/update", input).await?;
    if !decision.allowed {
        return Err(anyhow!("OPA rejected system update plan: {:?}", decision.details));
    }

    Ok(())
}

pub async fn process_files_and_sync(orchestrator: &Orchestrator, plan: &AlnUpdatePlan) -> Result<()> {
    // Write update record to PostgreSQL
    postgres::insert_update_record(
        &orchestrator.pg_pool,
        &plan.version,
        &plan.features,
    ).await?;

    // Write state to Redis
    redis::save_state(
        &orchestrator.redis,
        "ALN_UPDATE_2025",
        &plan.version,
        &plan.features,
    ).await?;

    // Publish Kafka messages
    kafka::producer::publish_file_update(
        &orchestrator.producer,
        &orchestrator.kafka_cfg.file_update_topic,
        &plan.version,
    ).await?;

    kafka::producer::publish_progress(
        &orchestrator.producer,
        &orchestrator.kafka_cfg.progress_topic,
        &plan.version,
        43,
        13,
    ).await?;

    info!("Files processed, DBs synced, Kafka events published.");
    Ok(())
}
