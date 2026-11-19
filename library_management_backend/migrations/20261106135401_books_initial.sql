CREATE TABLE IF NOT EXISTS books (
    id BLOB NOT NULL PRIMARY KEY,
    publisher_id BLOB NOT NULL,
    title TEXT NOT NULL,
    author TEXT NOT NULL,
    publication_year INTEGER NOT NULL,
    inventory_number TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT,
    deleted_at TEXT,
    CONSTRAINT publishers_fk FOREIGN KEY (publisher_id) REFERENCES publishers(id)
);

CREATE UNIQUE INDEX IF NOT EXISTS ux_books_inventory_number ON books(inventory_number);
CREATE INDEX IF NOT EXISTS ix_books_publisher_id ON books(publisher_id);
CREATE INDEX IF NOT EXISTS ix_books_title ON books(title);