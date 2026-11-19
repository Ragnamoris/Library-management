use std::sync::Arc;

use rusqlite::{params_from_iter, types::Value, OptionalExtension};
use time::UtcDateTime;

use crate::contracts::specifications::PublisherSpecification;
use crate::contracts::{PublishersStorage, StorageResult};
use crate::contracts_impl::converter::parse_record_lifetime;
use crate::contracts_impl::{storage_err, ManagedConnectionPool};
use crate::entities::value_objects::{Address, EntityId, Name};
use crate::entities::Publisher;
use crate::utils::{encode_datetime, encode_optional_datetime};

pub struct PublishersSqliteStorage {
    connection_pool: Arc<ManagedConnectionPool>,
}

impl PublishersSqliteStorage {
    pub fn new(connection_pool: Arc<ManagedConnectionPool>) -> Self {
        Self { connection_pool }
    }
}

impl PublishersStorage for PublishersSqliteStorage {
    async fn add(&self, publisher: Publisher) -> StorageResult<()> {
        const QUERY: &str = "INSERT INTO publishers(id, name, address, created_at, updated_at, deleted_at) VALUES(?1, ?2, ?3, ?4, ?5, ?6)";

        let client = self.connection_pool.acquire_connection().await?;
        let result = client
            .conn(move |connection| {
                let parameters: Vec<Value> = vec![
                    publisher.id.into(),
                    publisher.name.into(),
                    publisher.address.into(),
                    encode_datetime(publisher.record_lifetime.created_at()).into(),
                    encode_optional_datetime(publisher.record_lifetime.updated_at()).into(),
                    encode_optional_datetime(publisher.record_lifetime.deleted_at()).into(),
                ];
                connection.execute(QUERY, params_from_iter(parameters))
            })
            .await;
        self.connection_pool.return_connection(client).await?;
        result.map_err(storage_err)?;
        Ok(())
    }

    async fn get(&self, specification: PublisherSpecification) -> StorageResult<Option<Publisher>> {
        let mut filters: Vec<&str> = Vec::new();
        let mut parameters: Vec<Value> = Vec::new();

        if let Some(id) = specification.id {
            filters.push("id = ?");
            parameters.push(id.into());
        }
        if let Some(name) = specification.name {
            filters.push("name = ?");
            parameters.push(name.into());
        }
        if let Some(address) = specification.address {
            filters.push("address = ?");
            parameters.push(address.into());
        }

        if filters.is_empty() {
            return Ok(None);
        }

        let query: String = format!(
            "SELECT id, name, address, created_at, updated_at, deleted_at FROM publishers WHERE deleted_at IS NULL AND {}",
            filters.join(" AND ")
        );

        let client = self.connection_pool.acquire_connection().await?;
        let result = client
            .conn(move |connection| {
                let mut statement = connection.prepare(&query)?;
                statement
                    .query_row(params_from_iter(parameters), |row| {
                        let id: EntityId = row.get(0)?;
                        let name: Name = row.get(1)?;
                        let address: Address = row.get(2)?;
                        let record_lifetime = parse_record_lifetime(row, 3, 4, 5)?;
                        Ok(Publisher::create(id, name, address, record_lifetime))
                    })
                    .optional()
            })
            .await;
        self.connection_pool.return_connection(client).await?;
        let publisher = result.map_err(storage_err)?;
        Ok(publisher)
    }

    async fn update(&self, publisher: &Publisher) -> StorageResult<()> {
        const QUERY: &str = "UPDATE publishers SET name = ?1, address = ?2, updated_at = ?3 WHERE id = ?4 AND deleted_at IS NULL";

        let name = publisher.name.clone();
        let address = publisher.address.clone();
        let id = publisher.id;
        let updated_at = encode_datetime(UtcDateTime::now());

        let client = self.connection_pool.acquire_connection().await?;
        let result = client
            .conn(move |connection| {
                let parameters: Vec<Value> =
                    vec![name.into(), address.into(), updated_at.into(), id.into()];
                connection.execute(QUERY, params_from_iter(parameters))
            })
            .await;
        self.connection_pool.return_connection(client).await?;
        result.map_err(storage_err)?;
        Ok(())
    }

    async fn remove(&self, publisher: Publisher) -> StorageResult<()> {
        const QUERY: &str =
            "UPDATE publishers SET deleted_at = ?1 WHERE id = ?2 AND deleted_at IS NULL";

        let id = publisher.id;

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
}
