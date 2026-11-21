use std::sync::Arc;

use rusqlite::{params_from_iter, types::Value, OptionalExtension};
use time::UtcDateTime;
use uuid::Uuid;

use crate::contracts::specifications::LoanSpecification;
use crate::contracts::{LoansStorage, StorageError, StorageResult};
use crate::contracts_impl::converter::{
    parse_optional_utc_datetime, parse_record_lifetime, parse_utc_datetime,
};
use crate::contracts_impl::query::QueryFilter;
use crate::contracts_impl::{storage_err, ManagedConnectionPool};
use crate::entities::loan::MAX_ACTIVE_LOANS;
use crate::entities::value_objects::EntityId;
use crate::entities::Loan;
use crate::utils::{encode_datetime, encode_optional_datetime};

const SELECT: &str = "SELECT id, book_id, reader_id, issued_at, due_at, returned_at, created_at, updated_at, deleted_at FROM loans";

pub struct LoansSqliteStorage {
    connection_pool: Arc<ManagedConnectionPool>,
}

impl LoansSqliteStorage {
    pub fn new(connection_pool: Arc<ManagedConnectionPool>) -> Self {
        Self { connection_pool }
    }

    fn filter(specification: &LoanSpecification) -> QueryFilter {
        let mut filter = QueryFilter::new();
        if let Some(id) = specification.id {
            filter.eq("id", id);
        }
        if let Some(book_id) = specification.book_id {
            filter.eq("book_id", book_id);
        }
        if let Some(reader_id) = specification.reader_id {
            filter.eq("reader_id", reader_id);
        }
        if specification.only_active {
            filter.raw("returned_at IS NULL");
        }
        if specification.only_overdue {
            filter.raw_with(
                "returned_at IS NULL AND due_at < ?",
                encode_datetime(UtcDateTime::now()),
            );
        }
        if let Some(from) = specification.issued_from {
            filter.raw_with("issued_at >= ?", encode_datetime(from));
        }
        if let Some(to) = specification.issued_to {
            filter.raw_with("issued_at <= ?", encode_datetime(to));
        }
        filter
    }
}

fn map_row(row: &rusqlite::Row) -> rusqlite::Result<Loan> {
    let id: EntityId = row.get(0)?;
    let book_id: EntityId = row.get(1)?;
    let reader_id: EntityId = row.get(2)?;
    let issued_at = parse_utc_datetime(row, 3)?;
    let due_at = parse_utc_datetime(row, 4)?;
    let returned_at = parse_optional_utc_datetime(row, 5)?;
    let record_lifetime = parse_record_lifetime(row, 6, 7, 8)?;

    Loan::create(id, book_id, reader_id, issued_at, due_at, returned_at, record_lifetime).map_err(
        |error| {
            rusqlite::Error::FromSqlConversionFailure(
                4,
                rusqlite::types::Type::Text,
                Box::new(error),
            )
        },
    )
}

impl LoansStorage for LoansSqliteStorage {
    async fn add(&self, loan: Loan) -> StorageResult<()> {
        // Последний рубеж защиты правил абонемента на уровне хранения.
        // Основные проверки выполняет домен (Loan::issue).
        let active = self.count_active_for_reader(loan.reader_id.value()).await?;
        if active >= MAX_ACTIVE_LOANS {
            return Err(StorageError::BusinessRule(format!(
                "У читателя уже {active} книг на руках — достигнут предел в {MAX_ACTIVE_LOANS} книг"
            )));
        }
        if self.is_book_issued(loan.book_id.value()).await? {
            return Err(StorageError::BusinessRule(
                "Книга уже находится на руках и не может быть выдана повторно".to_string(),
            ));
        }

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

        let client = self.connection_pool.acquire_connection().await?;
        let result = client
            .conn(move |connection| {
                connection.execute(
                    "INSERT INTO loans(id, book_id, reader_id, issued_at, due_at, returned_at, created_at, updated_at, deleted_at)
                     VALUES(?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                    params_from_iter(parameters),
                )
            })
            .await;
        self.connection_pool.return_connection(client).await?;
        result.map_err(storage_err)?;
        Ok(())
    }

    async fn get(&self, specification: LoanSpecification) -> StorageResult<Option<Loan>> {
        let filter = Self::filter(&specification);
        if filter.is_empty() {
            return Ok(None);
        }
        let query = format!("{SELECT}{}", filter.where_clause());
        let parameters = filter.parameters();

        let client = self.connection_pool.acquire_connection().await?;
        let result = client
            .conn(move |connection| {
                connection
                    .prepare(&query)?
                    .query_row(params_from_iter(parameters), map_row)
                    .optional()
            })
            .await;
        self.connection_pool.return_connection(client).await?;
        result.map_err(storage_err)
    }

    async fn list(&self, specification: LoanSpecification) -> StorageResult<Vec<Loan>> {
        let filter = Self::filter(&specification);
        let query = format!(
            "{SELECT}{}{}",
            filter.where_clause(),
            filter.suffix("issued_at DESC", specification.limit, specification.offset)
        );
        let parameters = filter.parameters();

        let client = self.connection_pool.acquire_connection().await?;
        let result = client
            .conn(move |connection| {
                let mut statement = connection.prepare(&query)?;
                let rows = statement.query_map(params_from_iter(parameters), map_row)?;
                rows.collect::<rusqlite::Result<Vec<Loan>>>()
            })
            .await;
        self.connection_pool.return_connection(client).await?;
        result.map_err(storage_err)
    }

    async fn count(&self, specification: LoanSpecification) -> StorageResult<i64> {
        let filter = Self::filter(&specification);
        let query = format!("SELECT COUNT(*) FROM loans{}", filter.where_clause());
        let parameters = filter.parameters();

        let client = self.connection_pool.acquire_connection().await?;
        let result = client
            .conn(move |connection| {
                connection.query_row(&query, params_from_iter(parameters), |row| row.get(0))
            })
            .await;
        self.connection_pool.return_connection(client).await?;
        result.map_err(storage_err)
    }

    async fn update(&self, loan: &Loan) -> StorageResult<()> {
        let parameters: Vec<Value> = vec![
            loan.book_id.into(),
            loan.reader_id.into(),
            encode_datetime(loan.issued_at).into(),
            encode_datetime(loan.due_at).into(),
            encode_optional_datetime(loan.returned_at).into(),
            encode_datetime(UtcDateTime::now()).into(),
            loan.id.into(),
        ];

        let client = self.connection_pool.acquire_connection().await?;
        let result = client
            .conn(move |connection| {
                connection.execute(
                    "UPDATE loans SET book_id = ?1, reader_id = ?2, issued_at = ?3, due_at = ?4,
                                      returned_at = ?5, updated_at = ?6
                     WHERE id = ?7 AND deleted_at IS NULL",
                    params_from_iter(parameters),
                )
            })
            .await;
        self.connection_pool.return_connection(client).await?;
        result.map_err(storage_err)?;
        Ok(())
    }

    async fn remove(&self, loan: Loan) -> StorageResult<()> {
        let parameters: Vec<Value> = vec![encode_datetime(UtcDateTime::now()).into(), loan.id.into()];

        let client = self.connection_pool.acquire_connection().await?;
        let result = client
            .conn(move |connection| {
                connection.execute(
                    "UPDATE loans SET deleted_at = ?1 WHERE id = ?2 AND deleted_at IS NULL",
                    params_from_iter(parameters),
                )
            })
            .await;
        self.connection_pool.return_connection(client).await?;
        result.map_err(storage_err)?;
        Ok(())
    }

    async fn count_active_for_reader(&self, reader_id: Uuid) -> StorageResult<i64> {
        let parameters: Vec<Value> = vec![reader_id.into()];

        let client = self.connection_pool.acquire_connection().await?;
        let result = client
            .conn(move |connection| {
                connection.query_row(
                    "SELECT COUNT(*) FROM loans
                     WHERE reader_id = ?1 AND returned_at IS NULL AND deleted_at IS NULL",
                    params_from_iter(parameters),
                    |row| row.get(0),
                )
            })
            .await;
        self.connection_pool.return_connection(client).await?;
        result.map_err(storage_err)
    }

    async fn is_book_issued(&self, book_id: Uuid) -> StorageResult<bool> {
        let parameters: Vec<Value> = vec![book_id.into()];

        let client = self.connection_pool.acquire_connection().await?;
        let result = client
            .conn(move |connection| {
                connection.query_row(
                    "SELECT COUNT(*) FROM loans
                     WHERE book_id = ?1 AND returned_at IS NULL AND deleted_at IS NULL",
                    params_from_iter(parameters),
                    |row| row.get::<_, i64>(0),
                )
            })
            .await;
        self.connection_pool.return_connection(client).await?;
        let count = result.map_err(storage_err)?;
        Ok(count > 0)
    }
}