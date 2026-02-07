CREATE TABLE IF NOT EXISTS aln_update_data (
    id              BIGSERIAL PRIMARY KEY,
    version         TEXT NOT NULL,
    data            JSONB NOT NULL,
    metadata        JSONB DEFAULT '{}'::jsonb,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_aln_update_data_version
    ON aln_update_data (version);

CREATE INDEX IF NOT EXISTS idx_aln_update_data_created_at
    ON aln_update_data (created_at DESC);
