CREATE TABLE IF NOT EXISTS builds (
    id           BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    platform     TEXT NOT NULL,
    version      TEXT NOT NULL,
    channel      TEXT NOT NULL DEFAULT 'stable',
    filename     TEXT NOT NULL,
    size_bytes   BIGINT NOT NULL CHECK (size_bytes >= 0),
    sha256       CHAR(64) NOT NULL,
    s3_key       TEXT NOT NULL,
    content_type TEXT,
    commit_sha   TEXT,
    ci_run_id    TEXT,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_builds_latest ON builds (platform, channel, created_at DESC);
