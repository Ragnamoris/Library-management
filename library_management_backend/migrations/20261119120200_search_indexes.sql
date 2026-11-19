CREATE INDEX IF NOT EXISTS ix_books_author ON books(author);
CREATE INDEX IF NOT EXISTS ix_readers_full_name ON readers(full_name);
CREATE INDEX IF NOT EXISTS ix_loans_issued_at ON loans(issued_at);
CREATE INDEX IF NOT EXISTS ix_staff_role ON staff(role);
CREATE INDEX IF NOT EXISTS ix_report_snapshots_kind ON report_snapshots(kind, created_at);

CREATE INDEX IF NOT EXISTS ix_loans_active_due
    ON loans(due_at)
    WHERE returned_at IS NULL AND deleted_at IS NULL;