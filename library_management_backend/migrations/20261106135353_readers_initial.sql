CREATE TABLE IF NOT EXISTS readers (
    id BLOB NOT NULL PRIMARY KEY,
    full_name TEXT NOT NULL,
    phone_number TEXT NOT NULL,
    card_number TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT,
    deleted_at TEXT
);

CREATE UNIQUE INDEX IF NOT EXISTS ux_readers_phone_number ON readers(phone_number);
CREATE UNIQUE INDEX IF NOT EXISTS ux_readers_card_number ON readers(card_number);