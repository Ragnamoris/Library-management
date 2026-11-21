use std::sync::Arc;

use rusqlite::{params_from_iter, types::Value, OptionalExtension};
use time::UtcDateTime;

use crate::contracts::specifications::PublisherSpecification;
use crate::contracts::{PublishersStorage, StorageResult};
use crate::contracts_impl::converter::parse_record_lifetime;
use crate::contracts_impl::query::QueryFilter;
use crate::contracts_impl::{storage_err, ManagedConnectionPool};
use crate::entities::value_objects::{Address, EntityId, Name};
use crate::entities::Publisher;
use crate::utils::{encode_datetime, encode_optional_datetime};

const SELECT: &str = "SELECT id, name, address, created_at, updated_at, deleted_at FROM publishers";

pub struct PublishersSqliteStorage {
    connection_pool: Arc<ManagedConnectionPool>,
}

impl PublishersSqliteStorage {
    pub fn new(connection_pool: Arc<ManagedConnectionPool>) -> Self {
        Self { connection_pool }
    }

    fn filter(specification: &PublisherSpecification) -> QueryFilter {
        let mut filter = QueryFilter::new();
        if let Some(id) = specification.id {
            filter.eq("id", id);
        }
        if let Some(name) = &specification.name {
            filter.eq("name", name.value().to_string());
        }
        if let Some(address) = &specification.address {
            filter.eq("address", address.value().to_string());
        }
        if let Some(search) = &specification.search {
            filter.search(&["name", "address"], search);
        }
        filter
    }
}

fn map_row(row: &rusqlite::Row) -> rusqlite::Result<Publisher> {
    let id: EntityId = row.get(0)?;
    let name: Name = row.get(1)?;
    let address: Address = row.get(2)?;
    let record_lifetime = parse_record_lifetime(row, 3, 4, 5)?;
    Ok(Publisher::create(id, name, address, record_lifetime))
}

impl PublishersStorage for PublishersSqliteStorage {
    async fn add(&self, publisher: Publisher) -> StorageResult<()> {
        let parameters: Vec<Value> = vec![
            publisher.id.into(),
            publisher.name.into(),
            publisher.address.into(),
            encode_datetime(publisher.record_lifetime.created_at()).into(),
            encode_optional_datetime(publisher.record_lifetime.updated_at()).into(),
            encode_optional_datetime(publisher.record_lifetime.deleted_at()).into(),
        ];

        let client = self.connection_pool.acquire_connection().await?;
        let result = client
            .conn(move |connection| {
                connection.execute(
                    "INSERT INTO publishers(id, name, address, created_at, updated_at, deleted_at)
                     VALUES(?1, ?2, ?3, ?4, ?5, ?6)",
                    params_from_iter(parameters),
                )
            })
            .await;
        self.connection_pool.return_connection(client).await?;
        result.map_err(storage_err)?;
        Ok(())
    }

    async fn get(
        &self,
        specification: PublisherSpecification,
    ) -> StorageResult<Option<Publisher>> {
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

    async fn list(&self, specification: PublisherSpecification) -> StorageResult<Vec<Publisher>> {
        let filter = Self::filter(&specification);
        let query = format!(
            "{SELECT}{}{}",
            filter.where_clause(),
            filter.suffix("name", specification.limit, specification.offset)
        );
        let parameters = filter.parameters();

        let client = self.connection_pool.acquire_connection().await?;
        let result = client
            .conn(move |connection| {
                let mut statement = connection.prepare(&query)?;
                let rows = statement.query_map(params_from_iter(parameters), map_row)?;
                rows.collect::<rusqlite::Result<Vec<Publisher>>>()
            })
            .await;
        self.connection_pool.return_connection(client).await?;
        result.map_err(storage_err)
    }

    async fn count(&self, specification: PublisherSpecification) -> StorageResult<i64> {
        let filter = Self::filter(&specification);
        let query = format!("SELECT COUNT(*) FROM publishers{}", filter.where_clause());
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

    async fn update(&self, publisher: &Publisher) -> StorageResult<()> {
        let parameters: Vec<Value> = vec![
            publisher.name.clone().into(),
            publisher.address.clone().into(),
            encode_datetime(UtcDateTime::now()).into(),
            publisher.id.into(),
        ];

        let client = self.connection_pool.acquire_connection().await?;
        let result = client
            .conn(move |connection| {
                connection.execute(
                    "UPDATE publishers SET name = ?1, address = ?2, updated_at = ?3
                     WHERE id = ?4 AND deleted_at IS NULL",
                    params_from_iter(parameters),
                )
            })
            .await;
        self.connection_pool.return_connection(client).await?;
        result.map_err(storage_err)?;
        Ok(())
    }

    async fn remove(&self, publisher: Publisher) -> StorageResult<()> {
        let parameters: Vec<Value> = vec![
            encode_datetime(UtcDateTime::now()).into(),
            publisher.id.into(),
        ];

        let client = self.connection_pool.acquire_connection().await?;
        let result = client
            .conn(move |connection| {
                connection.execute(
                    "UPDATE publishers SET deleted_at = ?1 WHERE id = ?2 AND deleted_at IS NULL",
                    params_from_iter(parameters),
                )
            })
            .await;
        self.connection_pool.return_connection(client).await?;
        result.map_err(storage_err)?;
        Ok(())
    }
}