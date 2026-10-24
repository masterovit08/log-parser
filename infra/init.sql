DROP TABLE IF EXISTS jobs CASCADE;

CREATE TABLE jobs (
      id          UUID NOT NULL PRIMARY KEY,
      status      VARCHAR(32) NOT NULL DEFAULT 'queued',
      minio_key   TEXT NOT NULL,
      created_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
      started_at  TIMESTAMPTZ,
      finished_at TIMESTAMPTZ
);