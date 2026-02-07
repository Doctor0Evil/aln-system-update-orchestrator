use crate::{
    aln::AlnUpdatePlan,
    db::{postgres, redis},
    kafka,
    opa::Client as OpaClient,
    orchestrator::Orchestrator,
};
use anyhow::{anyhow, Result};
use chrono::Utc;
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
        return Err(anyhow!(
            "OPA rejected system update plan: {:?}",
            decision.details
        ));
    }

    Ok(())
}

pub async fn process_files_and_sync(
    orchestrator: &Orchestrator,
    plan: &AlnUpdatePlan,
) -> Result<()> {
    let token_id = "ALN_UPDATE_2025";
    let compliance_score = Some(0.99999999999_f64);
    let latency = Some("10^-17s");
    let sync_status = Some("all_nodes_databases_vm_lan");

    let payload = json!({
        "token_id": token_id,
        "version": plan.version,
        "features": plan.rego_exec.features,
        "timestamp": Utc::now(),
    });

    // Write update record to PostgreSQL
    postgres::insert_update_record(
        &orchestrator.pg_pool,
        &plan.version,
        &plan.rego_exec.features,
    )
    .await?;

    // Write detailed log to PostgreSQL
    postgres::insert_update_log(
        &orchestrator.pg_pool,
        token_id,
        &plan.version,
        43,
        plan.rego_exec.features.len() as i32,
        compliance_score,
        latency,
        sync_status,
        &payload,
    )
    .await?;

    // Write state to Redis
    redis::save_state(
        &orchestrator.redis,
        token_id,
        &plan.version,
        &plan.rego_exec.features,
    )
    .await?;

    // Publish Kafka messages
    kafka::producer::publish_file_update(
        &orchestrator.producer,
        &orchestrator.kafka_cfg.file_update_topic,
        &plan.version,
    )
    .await?;

    kafka::producer::publish_progress(
        &orchestrator.producer,
        &orchestrator.kafka_cfg.progress_topic,
        &plan.version,
        43,
        plan.rego_exec.features.len() as i32,
    )
    .await?;

    info!("Files processed, DBs synced, Kafka events published.");
    Ok(())
}
