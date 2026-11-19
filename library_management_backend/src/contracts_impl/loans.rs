use std::sync::Arc;

use rusqlite::{params_from_iter, types::Value, OptionalExtension};
use time::UtcDateTime;
use uuid::Uuid;

use crate::contracts::specifications::LoanSpecification;
use crate::contracts::{LoansStorage, StorageError, StorageResult};
use crate::contracts_impl::converter::{
    parse_optional_utc_datetime, parse_record_lifetime, parse_utc_datetime,
};
use crate::contracts_impl::{storage_err, ManagedConnectionPool};
use crate::entities::loan::MAX_ACTIVE_LOANS;
use crate::entities::value_objects::EntityId;
use crate::entities::Loan;
use crate::utils::{encode_datetime, encode_optional_datetime};

pub struct LoansSqliteStorage {
    connection_pool: Arc<ManagedConnectionPool>,
}

impl LoansSqliteStorage {
    pub fn new(connection_pool: Arc<ManagedConnectionPool>) -> Self {
        Self { connection_pool }
    }
}

impl LoansStorage for LoansSqliteStorage {
    async fn add(&self, loan: Loan) -> StorageResult<()> {
        const INSERT_QUERY: &str = "INSERT INTO loans(id, book_id, reader_id, issued_at, due_at, returned_at, created_at, updated_at, deleted_at) VALUES(?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)";

        let active = self.count_active_for_reader(loan.reader_id.value()).await?;
        if active >= MAX_ACTIVE_LOANS {
            return Err(StorageError::from(
                "reader already has the maximum number of books",
            ));
        }

        let issued = self.is_book_issued(loan.book_id.value()).await?;
        if issued {
            return Err(StorageError::from(
                "book is already issued to another reader",
            ));
        }

        let client = self.connection_pool.acquire_connection().await?;
        let result = client
            .conn(move |connection| {
                let parameters: Vec<Value> = vec![
                    loan.id.into(),
                    loan.book_id.into(),
                    loan.reader_id.into(),
                    encode_datetime(loan.issued_at).into(),
                    encode_datetime(loan.due_at).into(),
                    encode_optional_datetime(loan.returned_at).into(),
                    encode_datetime(loan.record_lifetime.created_at()).into(),
                    encode_optional_datetime(loan.record_lifetime.updated_at()).into(),
                    encode_optional_datetime(loan.record_lifetime.deleted_at()).into(),
                ];
                connection.execute(INSERT_QUERY, params_from_iter(parameters))
            })
            .await;
        self.connection_pool.return_connection(client).await?;
        result.map_err(storage_err)?;
        Ok(())
    }

    async fn get(&self, specification: LoanSpecification) -> StorageResult<Option<Loan>> {
        let mut filters: Vec<&str> = Vec::new();
        let mut parameters: Vec<Value> = Vec::new();

        if let Some(id) = specification.id {
            filters.push("id = ?");
            parameters.push(id.into());
        }
        if let Some(book_id) = specification.book_id {
            filters.push("book_id = ?");
            parameters.push(book_id.into());
        }
        if let Some(reader_id) = specification.reader_id {
            filters.push("reader_id = ?");
            parameters.push(reader_id.into());
        }
        if specification.only_active {
            filters.push("returned_at IS NULL");
        }

        if filters.is_empty() {
            return Ok(None);
        }

        let query: String = format!(
            "SELECT id, book_id, reader_id, issued_at, due_at, returned_at, created_at, updated_at, deleted_at FROM loans WHERE deleted_at IS NULL AND {}",
            filters.join(" AND ")
        );

        let client = self.connection_pool.acquire_connection().await?;
        let result = client
            .conn(move |connection| {
                let mut statement = connection.prepare(&query)?;
                statement
                    .query_row(params_from_iter(parameters), |row| {
                        let id: EntityId = row.get(0)?;
                        let book_id: EntityId = row.get(1)?;
                        let reader_id: EntityId = row.get(2)?;
                        let issued_at = parse_utc_datetime(row, 3)?;
                        let due_at = parse_utc_datetime(row, 4)?;
                        let returned_at = parse_optional_utc_datetime(row, 5)?;
                        let record_lifetime = parse_record_lifetime(row, 6, 7, 8)?;
                        Loan::create(
                            id,
                            book_id,
                            reader_id,
                            issued_at,
                            due_at,
                            returned_at,
                            record_lifetime,
                        )
                        .map_err(|error| {
                            rusqlite::Error::FromSqlConversionFailure(
                                4,
                                rusqlite::types::Type::Text,
                                Box::new(error),
                            )
                        })
                    })
                    .optional()
            })
            .await;
        self.connection_pool.return_connection(client).await?;
        let loan = result.map_err(storage_err)?;
        Ok(loan)
    }

    async fn update(&self, loan: &Loan) -> StorageResult<()> {
        const QUERY: &str = "UPDATE loans SET book_id = ?1, reader_id = ?2, issued_at = ?3, due_at = ?4, returned_at = ?5, updated_at = ?6 WHERE id = ?7 AND deleted_at IS NULL";

        let book_id = loan.book_id;
        let reader_id = loan.reader_id;
        let issued_at = loan.issued_at;
        let due_at = loan.due_at;
        let returned_at = loan.returned_at;
        let id = loan.id;
        let updated_at = encode_datetime(UtcDateTime::now());

        let client = self.connection_pool.acquire_connection().await?;
        let result = client
            .conn(move |connection| {
                let parameters: Vec<Value> = vec![
                    book_id.into(),
                    reader_id.into(),
                    encode_datetime(issued_at).into(),
                    encode_datetime(due_at).into(),
                    encode_optional_datetime(returned_at).into(),
                    updated_at.into(),
                    id.into(),
                ];
                connection.execute(QUERY, params_from_iter(parameters))
            })
            .await;
        self.connection_pool.return_connection(client).await?;
        result.map_err(storage_err)?;
        Ok(())
    }

    async fn remove(&self, loan: Loan) -> StorageResult<()> {
        const QUERY: &str = "UPDATE loans SET deleted_at = ?1 WHERE id = ?2 AND deleted_at IS NULL";

        let id = loan.id;

        let client = self.connection_pool.acquire_connection().await?;
        let result = client
            .conn(move |connection| {
                let parameters: Vec<Value> =
                    vec![encode_datetime(UtcDateTime::now()).into(), id.into()];
                connection.execute(QUERY, params_from_iter(parameters))
            })
            .await;
        self.connection_pool.return_connection(client).await?;
        result.map_err(storage_err)?;
        Ok(())
    }

    async fn count_active_for_reader(&self, reader_id: Uuid) -> StorageResult<i64> {
        const QUERY: &str = "SELECT COUNT(*) FROM loans WHERE reader_id = ?1 AND returned_at IS NULL AND deleted_at IS NULL";

        let parameters: Vec<Value> = vec![reader_id.into()];

        let client = self.connection_pool.acquire_connection().await?;
        let result = client
            .conn(move |connection| {
                connection.query_row(QUERY, params_from_iter(parameters), |row| {
                    let value: i64 = row.get(0)?;
                    Ok(value)
                })
            })
            .await;
        self.connection_pool.return_connection(client).await?;
        let count = result.map_err(storage_err)?;
        Ok(count)
    }

    async fn is_book_issued(&self, book_id: Uuid) -> StorageResult<bool> {
        const QUERY: &str = "SELECT COUNT(*) FROM loans WHERE book_id = ?1 AND returned_at IS NULL AND deleted_at IS NULL";

        let parameters: Vec<Value> = vec![book_id.into()];

        let client = self.connection_pool.acquire_connection().await?;
        let result = client
            .conn(move |connection| {
                connection.query_row(QUERY, params_from_iter(parameters), |row| {
                    let value: i64 = row.get(0)?;
                    Ok(value)
                })
            })
            .await;
        self.connection_pool.return_connection(client).await?;
        let count = result.map_err(storage_err)?;
        Ok(count > 0)
    }
}
