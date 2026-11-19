CREATE TABLE IF NOT EXISTS staff (
    id BLOB NOT NULL PRIMARY KEY,
    login TEXT NOT NULL,
    password_hash TEXT NOT NULL,
    full_name TEXT NOT NULL,
    role TEXT NOT NULL CHECK (role IN ('admin', 'librarian')),
    created_at TEXT NOT NULL,
    updated_at TEXT,
    deleted_at TEXT
);

CREATE UNIQUE INDEX IF NOT EXISTS ux_staff_login ON staff(login);