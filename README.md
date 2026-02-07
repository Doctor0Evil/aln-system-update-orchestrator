# ALN System Update Orchestrator

ALN System Update Orchestrator is a Rust-based service that executes the `@ALN_SYSTEM_UPDATE` pipeline as a real, zero-trust, policy-as-code update engine for GitHub-backed systems.

## Features

- Parses `.aln` update specs and orchestrates:
  - File processing.
  - GitHub integration and commits.
  - VM deployment hooks.
  - Database and cache synchronization.
- Publishes and consumes Kafka topics:
  - `aln_file_update` for file update events.
  - `aln_update_progress` for progress metrics.
- Persists update metadata to:
  - PostgreSQL table `aln_update_data`.
  - PostgreSQL table `update_log_v1.7`.
  - Redis keys `aln_update_state_1.0.1.7:{token_id}`.
- Uses OPA / Rego (`system_update_policy_v1.7.rego`) to validate:
  - Repo structure.
  - Version history tracking.
  - K8s manifest scaling rules.

## Running locally

```bash
cargo build --release
./target/release/aln-system-update-orchestrator


Configure Kafka, PostgreSQL, and Redis via config/*.toml.

text

***

### `config/kafka.toml`

```toml
bootstrap_servers = "localhost:9092"
file_update_topic = "aln_file_update"
progress_topic = "aln_update_progress"
group_id = "aln-system-update-orchestrator"

aln-system-update-orchestrator/
├─ Cargo.toml
├─ Dockerfile
├─ README.md
├─ aln/
│  ├─ system_update_integration_v1.7.aln
│  └─ system_update_policy_v1.7.rego
├─ config/
│  ├─ kafka.toml
│  ├─ postgres.toml
│  └─ redis.toml
├─ k8s/
│  ├─ deployment.yaml
│  └─ service.yaml
└─ src/
   ├─ main.rs
   ├─ lib.rs
   ├─ aln/
   │  ├─ mod.rs
   │  └─ parser.rs
   ├─ orchestrator/
   │  ├─ mod.rs
   │  └─ steps.rs
   ├─ kafka/
   │  ├─ mod.rs
   │  ├─ producer.rs
   │  └─ consumer.rs
   ├─ db/
   │  ├─ mod.rs
   │  ├─ postgres.rs
   │  └─ redis.rs
   └─ opa/
      ├─ mod.rs
      └─ client.rs
