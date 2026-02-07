CREATE TABLE IF NOT EXISTS update_log_v1_7 (
    id                   BIGSERIAL PRIMARY KEY,
    token_id             TEXT NOT NULL,
    version              TEXT NOT NULL,
    files_processed      INTEGER NOT NULL,
    features_added       INTEGER NOT NULL,
    compliance_score     NUMERIC(12, 11),
    latency              TEXT,
    sync_status          TEXT,
    raw_payload          JSONB,
    created_at           TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_update_log_v1_7_token
    ON update_log_v1_7 (token_id);

CREATE INDEX IF NOT EXISTS idx_update_log_v1_7_version
    ON update_log_v1_7 (version);

CREATE INDEX IF NOT EXISTS idx_update_log_v1_7_created_at
    ON update_log_v1_7 (created_at DESC);
