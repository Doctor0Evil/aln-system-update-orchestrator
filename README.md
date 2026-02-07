# aln-system-update-orchestrator
A  Rust + Rego + ALN repository that implements the full @ALN_SYSTEM_UPDATE pipeline as a real service.  Rust core:  Reads .aln spec files, parses update plans, orchestrates steps (file processing, GitHub sync, VM deploy, DB sync).  Kafka producer/consumer for aln_file_update and aln_update_progress topics.
