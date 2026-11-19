CREATE TABLE IF NOT EXISTS report_snapshots (
    id BLOB NOT NULL PRIMARY KEY,
    kind TEXT NOT NULL CHECK (kind IN ('books_on_hands', 'overdue')),
    period_from TEXT NOT NULL,
    period_to TEXT NOT NULL,
    staff_id BLOB NOT NULL,
    payload TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT,
    deleted_at TEXT,
    CONSTRAINT staff_fk FOREIGN KEY (staff_id) REFERENCES staff(id)
);