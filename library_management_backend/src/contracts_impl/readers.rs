use std::sync::Arc;

use rusqlite::{params_from_iter, types::Value, OptionalExtension};
use time::UtcDateTime;

use crate::contracts::specifications::ReaderSpecification;
use crate::contracts::{ReadersStorage, StorageResult};
use crate::contracts_impl::converter::parse_record_lifetime;
use crate::contracts_impl::query::QueryFilter;
use crate::contracts_impl::{storage_err, ManagedConnectionPool};
use crate::entities::value_objects::{CardNumber, EntityId, Name, PhoneNumber};
use crate::entities::Reader;
use crate::utils::{encode_datetime, encode_optional_datetime};

const SELECT: &str =
    "SELECT id, full_name, phone_number, card_number, created_at, updated_at, deleted_at FROM readers";

pub struct ReadersSqliteStorage {
    connection_pool: Arc<ManagedConnectionPool>,
}

impl ReadersSqliteStorage {
    pub fn new(connection_pool: Arc<ManagedConnectionPool>) -> Self {
        Self { connection_pool }
    }

    fn filter(specification: &ReaderSpecification) -> QueryFilter {
        let mut filter = QueryFilter::new();
        if let Some(id) = specification.id {
            filter.eq("id", id);
        }
        if let Some(full_name) = &specification.full_name {
            filter.eq("full_name", full_name.value().to_string());
        }
        if let Some(phone_number) = &specification.phone_number {
            filter.eq("phone_number", phone_number.value().to_string());
        }
        if let Some(card_number) = &specification.card_number {
            filter.eq("card_number", card_number.value().to_string());
        }
        if let Some(search) = &specification.search {
            filter.search(&["full_name", "card_number", "phone_number"], search);
        }
        filter
    }
}

fn map_row(row: &rusqlite::Row) -> rusqlite::Result<Reader> {
    let id: EntityId = row.get(0)?;
    let full_name: Name = row.get(1)?;
    let phone_number: PhoneNumber = row.get(2)?;
    let card_number: CardNumber = row.get(3)?;
    let record_lifetime = parse_record_lifetime(row, 4, 5, 6)?;
    Ok(Reader::create(id, full_name, phone_number, card_number, record_lifetime))
}

impl ReadersStorage for ReadersSqliteStorage {
    async fn add(&self, reader: Reader) -> StorageResult<()> {
        let parameters: Vec<Value> = vec![
            reader.id.into(),
            reader.full_name.into(),
            reader.phone_number.into(),
            reader.card_number.into(),
            encode_datetime(reader.record_lifetime.created_at()).into(),
            encode_optional_datetime(reader.record_lifetime.updated_at()).into(),
            encode_optional_datetime(reader.record_lifetime.deleted_at()).into(),
        ];

        let client = self.connection_pool.acquire_connection().await?;
        let result = client
            .conn(move |connection| {
                connection.execute(
                    "INSERT INTO readers(id, full_name, phone_number, card_number, created_at, updated_at, deleted_at)
                     VALUES(?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                    params_from_iter(parameters),
                )
            })
            .await;
        self.connection_pool.return_connection(client).await?;
        result.map_err(storage_err)?;
        Ok(())
    }

    async fn get(&self, specification: ReaderSpecification) -> StorageResult<Option<Reader>> {
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

    async fn list(&self, specification: ReaderSpecification) -> StorageResult<Vec<Reader>> {
        let filter = Self::filter(&specification);
        let query = format!(
            "{SELECT}{}{}",
            filter.where_clause(),
            filter.suffix("full_name", specification.limit, specification.offset)
        );
        let parameters = filter.parameters();

        let client = self.connection_pool.acquire_connection().await?;
        let result = client
            .conn(move |connection| {
                let mut statement = connection.prepare(&query)?;
                let rows = statement.query_map(params_from_iter(parameters), map_row)?;
                rows.collect::<rusqlite::Result<Vec<Reader>>>()
            })
            .await;
        self.connection_pool.return_connection(client).await?;
        result.map_err(storage_err)
    }

    async fn count(&self, specification: ReaderSpecification) -> StorageResult<i64> {
        let filter = Self::filter(&specification);
        let query = format!("SELECT COUNT(*) FROM readers{}", filter.where_clause());
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

    async fn update(&self, reader: &Reader) -> StorageResult<()> {
        let parameters: Vec<Value> = vec![
            reader.full_name.clone().into(),
            reader.phone_number.clone().into(),
            reader.card_number.clone().into(),
            encode_datetime(UtcDateTime::now()).into(),
            reader.id.into(),
        ];

        let client = self.connection_pool.acquire_connection().await?;
        let result = client
            .conn(move |connection| {
                connection.execute(
                    "UPDATE readers SET full_name = ?1, phone_number = ?2, card_number = ?3, updated_at = ?4
                     WHERE id = ?5 AND deleted_at IS NULL",
                    params_from_iter(parameters),
                )
            })
            .await;
        self.connection_pool.return_connection(client).await?;
        result.map_err(storage_err)?;
        Ok(())
    }

    async fn remove(&self, reader: Reader) -> StorageResult<()> {
        let parameters: Vec<Value> = vec![
            encode_datetime(UtcDateTime::now()).into(),
            reader.id.into(),
        ];

        let client = self.connection_pool.acquire_connection().await?;
        let result = client
            .conn(move |connection| {
                connection.execute(
                    "UPDATE readers SET deleted_at = ?1 WHERE id = ?2 AND deleted_at IS NULL",
                    params_from_iter(parameters),
                )
            })
            .await;
        self.connection_pool.return_connection(client).await?;
        result.map_err(storage_err)?;
        Ok(())
    }
}