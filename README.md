# aln-system-update-orchestrator
A  Rust + Rego + ALN repository that implements the full @ALN_SYSTEM_UPDATE pipeline as a real service.  Rust core:  Reads .aln spec files, parses update plans, orchestrates steps (file processing, GitHub sync, VM deploy, DB sync).  Kafka producer/consumer for aln_file_update and aln_update_progress topics.

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
