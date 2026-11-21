use rusqlite::Error as SqliteError;

use crate::contracts::StorageError;

// Расширенные коды результатов SQLite.
const SQLITE_CONSTRAINT_UNIQUE: i32 = 2067;
const SQLITE_CONSTRAINT_PRIMARYKEY: i32 = 1555;
const SQLITE_CONSTRAINT_FOREIGNKEY: i32 = 787;
const SQLITE_CONSTRAINT_CHECK: i32 = 275;
const SQLITE_CONSTRAINT_NOTNULL: i32 = 1299;

fn from_sqlite(error: &SqliteError) -> StorageError {
    match error {
        SqliteError::SqliteFailure(failure, message) => {
            let text = message.clone().unwrap_or_else(|| failure.to_string());
            match failure.extended_code {
                SQLITE_CONSTRAINT_UNIQUE | SQLITE_CONSTRAINT_PRIMARYKEY => {
                    StorageError::UniqueViolation(text)
                }
                SQLITE_CONSTRAINT_FOREIGNKEY => StorageError::ForeignKeyViolation(text),
                SQLITE_CONSTRAINT_CHECK => StorageError::CheckViolation(text),
                SQLITE_CONSTRAINT_NOTNULL => StorageError::NotNullViolation(text),
                _ => StorageError::Database(text),
            }
        }
        SqliteError::QueryReturnedNoRows => StorageError::NotFound,
        SqliteError::FromSqlConversionFailure(_, _, source) => {
            StorageError::Conversion(source.to_string())
        }
        other => StorageError::Database(other.to_string()),
    }
}

fn from_message(message: &str) -> StorageError {
    let lowered = message.to_lowercase();
    if lowered.contains("unique constraint") {
        StorageError::UniqueViolation(message.to_string())
    } else if lowered.contains("foreign key constraint") {
        StorageError::ForeignKeyViolation(message.to_string())
    } else if lowered.contains("check constraint") {
        StorageError::CheckViolation(message.to_string())
    } else if lowered.contains("not null constraint") {
        StorageError::NotNullViolation(message.to_string())
    } else {
        StorageError::Database(message.to_string())
    }
}

/// Единая точка преобразования ошибок драйвера в ошибки хранения.
pub(crate) fn storage_err<E: std::error::Error + 'static>(error: E) -> StorageError {
    let mut current: Option<&(dyn std::error::Error + 'static)> = Some(&error);
    while let Some(item) = current {
        if let Some(sqlite_error) = item.downcast_ref::<SqliteError>() {
            return from_sqlite(sqlite_error);
        }
        current = item.source();
    }
    from_message(&error.to_string())
}