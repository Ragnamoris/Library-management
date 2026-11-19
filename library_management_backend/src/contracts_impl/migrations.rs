use std::sync::Arc;

use rusqlite::params;
use time::UtcDateTime;

use crate::contracts::StorageResult;
use crate::contracts_impl::{storage_err, ManagedConnectionPool};
use crate::utils::encode_datetime;

pub const MIGRATIONS: &[(&str, &str)] = &[
    (
        "20261106135347_publishers_initial",
        include_str!("../../migrations/20261106135347_publishers_initial.sql"),
    ),
    (
        "20261106135353_readers_initial",
        include_str!("../../migrations/20261106135353_readers_initial.sql"),
    ),
    (
        "20261106135401_books_initial",
        include_str!("../../migrations/20261106135401_books_initial.sql"),
    ),
    (
        "20261106135411_loans_initial",
        include_str!("../../migrations/20261106135411_loans_initial.sql"),
    ),
    (
        "20261119120000_staff_initial",
        include_str!("../../migrations/20261119120000_staff_initial.sql"),
    ),
    (
        "20261119120100_report_snapshots_initial",
        include_str!("../../migrations/20261119120100_report_snapshots_initial.sql"),
    ),
    (
        "20261119120200_search_indexes",
        include_str!("../../migrations/20261119120200_search_indexes.sql"),
    ),
];

pub async fn run(pool: &Arc<ManagedConnectionPool>) -> StorageResult<usize> {
    let client = pool.acquire_connection().await?;

    let result = client
        .conn(|connection| -> rusqlite::Result<usize> {
            connection.execute_batch(
                "CREATE TABLE IF NOT EXISTS schema_migrations (
                    name TEXT NOT NULL PRIMARY KEY,
                    applied_at TEXT NOT NULL
                );",
            )?;

            let mut applied = 0usize;

            for &(name, sql) in MIGRATIONS {
                let exists: i64 = connection.query_row(
                    "SELECT EXISTS(SELECT 1 FROM schema_migrations WHERE name = ?1)",
                    params![name],
                    |row| row.get(0),
                )?;

                if exists == 1 {
                    println!("  = {name} (уже применена, пропуск)");
                    continue;
                }

                let transaction = connection.unchecked_transaction()?;
                transaction.execute_batch(sql)?;
                transaction.execute(
                    "INSERT INTO schema_migrations(name, applied_at) VALUES(?1, ?2)",
                    params![name, encode_datetime(UtcDateTime::now())],
                )?;
                transaction.commit()?;

                applied += 1;
                println!("  + {name} (применена)");
            }

            Ok(applied)
        })
        .await;

    pool.return_connection(client).await?;
    result.map_err(storage_err)
}

pub async fn report(pool: &Arc<ManagedConnectionPool>) -> StorageResult<()> {
    let client = pool.acquire_connection().await?;

    let result = client
        .conn(|connection| -> rusqlite::Result<Vec<String>> {
            let mut lines: Vec<String> = Vec::new();

            let mut statement = connection.prepare(
                "SELECT name FROM sqlite_master
                 WHERE type = 'table' AND name NOT LIKE 'sqlite_%'
                 ORDER BY name",
            )?;
            let tables: Vec<String> = statement
                .query_map([], |row| row.get::<_, String>(0))?
                .collect::<rusqlite::Result<Vec<String>>>()?;

            lines.push("Таблицы базы данных:".to_string());
            for table in tables {
                let count: i64 = connection.query_row(
                    &format!("SELECT COUNT(*) FROM \"{table}\""),
                    [],
                    |row| row.get(0),
                )?;
                lines.push(format!("  - {table:<18} записей: {count}"));
            }

            let indexes: i64 = connection.query_row(
                "SELECT COUNT(*) FROM sqlite_master
                 WHERE type = 'index' AND (name LIKE 'ix_%' OR name LIKE 'ux_%')",
                [],
                |row| row.get(0),
            )?;
            lines.push(format!("Индексов создано: {indexes}"));

            Ok(lines)
        })
        .await;

    pool.return_connection(client).await?;
    let lines = result.map_err(storage_err)?;
    for line in lines {
        println!("{line}");
    }
    Ok(())
}