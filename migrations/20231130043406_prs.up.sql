CREATE TABLE IF NOT EXISTS prs
(
    id         TEXT PRIMARY KEY,
    title      TEXT      NOT NULL,
    data       TEXT      NOT NULL,
    status     TEXT      NOT NULL,
    score      FLOAT     NOT NULL DEFAULT 0,
    opened_at  TIMESTAMP NOT NULL,
    created_at TIMESTAMP          DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP          DEFAULT CURRENT_TIMESTAMP
);


CREATE TABLE pr_sync_metadata
(
    owner          TEXT NOT NULL,
    repo           TEXT NOT NULL,
    last_cursor    TEXT,
    last_synced_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (owner, repo)
);