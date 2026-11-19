CREATE TABLE IF NOT EXISTS loans (
    id BLOB NOT NULL PRIMARY KEY,
    book_id BLOB NOT NULL,
    reader_id BLOB NOT NULL,
    issued_at TEXT NOT NULL,
    due_at TEXT NOT NULL,
    returned_at TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT,
    deleted_at TEXT,
    CONSTRAINT books_fk FOREIGN KEY (book_id) REFERENCES books(id),
    CONSTRAINT readers_fk FOREIGN KEY (reader_id) REFERENCES readers(id)
);

CREATE INDEX IF NOT EXISTS ix_loans_book_id ON loans(book_id);
CREATE INDEX IF NOT EXISTS ix_loans_reader_id ON loans(reader_id);
CREATE UNIQUE INDEX IF NOT EXISTS ux_loans_active_book
    ON loans(book_id)
    WHERE returned_at IS NULL AND deleted_at IS NULL;