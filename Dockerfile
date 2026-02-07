FROM alpine:3.20 AS base
RUN apk add --no-cache \
    ca-certificates \
    libc6-compat \
    opa \
    kafka \
    postgresql-client \
    redis

WORKDIR /app

# Copy orchestrator binary and policy bundle
COPY target/release/aln-system-update-orchestrator /usr/local/bin/aln-system-update-orchestrator
COPY aln/system_update_policy_v1.7.rego /app/policies/system_update_policy_v1.7.rego
COPY aln/system_update_integration_v1.7.aln /app/aln/system_update_integration_v1.7.aln

ENV ALN_VERSION=1.0.1.7
ENV OPA_ADDR=0.0.0.0:8181
ENV OPA_DATA_DIR=/app/policies

EXPOSE 8080
EXPOSE 8181

ENTRYPOINT ["/bin/sh", "-c", "\
  opa run --server --addr=${OPA_ADDR} ${OPA_DATA_DIR} & \
  aln-system-update-orchestrator \
"]
