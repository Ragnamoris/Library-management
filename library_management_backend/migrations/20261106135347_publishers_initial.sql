CREATE TABLE IF NOT EXISTS publishers (
    id BLOB NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    address TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT,
    deleted_at TEXT
);

CREATE UNIQUE INDEX IF NOT EXISTS ux_publishers_name ON publishers(name);