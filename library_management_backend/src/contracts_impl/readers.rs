use std::sync::Arc;

use rusqlite::{params_from_iter, types::Value, OptionalExtension};
use time::UtcDateTime;

use crate::contracts::specifications::ReaderSpecification;
use crate::contracts::{ReadersStorage, StorageResult};
use crate::contracts_impl::converter::parse_record_lifetime;
use crate::contracts_impl::{storage_err, ManagedConnectionPool};
use crate::entities::value_objects::{CardNumber, EntityId, Name, PhoneNumber};
use crate::entities::Reader;
use crate::utils::{encode_datetime, encode_optional_datetime};

pub struct ReadersSqliteStorage {
    connection_pool: Arc<ManagedConnectionPool>,
}

impl ReadersSqliteStorage {
    pub fn new(connection_pool: Arc<ManagedConnectionPool>) -> Self {
        Self { connection_pool }
    }
}

impl ReadersStorage for ReadersSqliteStorage {
    async fn add(&self, reader: Reader) -> StorageResult<()> {
        const QUERY: &str = "INSERT INTO readers(id, full_name, phone_number, card_number, created_at, updated_at, deleted_at) VALUES(?1, ?2, ?3, ?4, ?5, ?6, ?7)";

        let client = self.connection_pool.acquire_connection().await?;
        let result = client
            .conn(move |connection| {
                let parameters: Vec<Value> = vec![
                    reader.id.into(),
                    reader.full_name.into(),
                    reader.phone_number.into(),
                    reader.card_number.into(),
                    encode_datetime(reader.record_lifetime.created_at()).into(),
                    encode_optional_datetime(reader.record_lifetime.updated_at()).into(),
                    encode_optional_datetime(reader.record_lifetime.deleted_at()).into(),
                ];
                connection.execute(QUERY, params_from_iter(parameters))
            })
            .await;
        self.connection_pool.return_connection(client).await?;
        result.map_err(storage_err)?;
        Ok(())
    }

    async fn get(&self, specification: ReaderSpecification) -> StorageResult<Option<Reader>> {
        let mut filters: Vec<&str> = Vec::new();
        let mut parameters: Vec<Value> = Vec::new();

        if let Some(id) = specification.id {
            filters.push("id = ?");
            parameters.push(id.into());
        }
        if let Some(full_name) = specification.full_name {
            filters.push("full_name = ?");
            parameters.push(full_name.into());
        }
        if let Some(phone_number) = specification.phone_number {
            filters.push("phone_number = ?");
            parameters.push(phone_number.into());
        }
        if let Some(card_number) = specification.card_number {
            filters.push("card_number = ?");
            parameters.push(card_number.into());
        }

        if filters.is_empty() {
            return Ok(None);
        }

        let query: String = format!(
            "SELECT id, full_name, phone_number, card_number, created_at, updated_at, deleted_at FROM readers WHERE deleted_at IS NULL AND {}",
            filters.join(" AND ")
        );

        let client = self.connection_pool.acquire_connection().await?;
        let result = client
            .conn(move |connection| {
                let mut statement = connection.prepare(&query)?;
                statement
                    .query_row(params_from_iter(parameters), |row| {
                        let id: EntityId = row.get(0)?;
                        let full_name: Name = row.get(1)?;
                        let phone_number: PhoneNumber = row.get(2)?;
                        let card_number: CardNumber = row.get(3)?;
                        let record_lifetime = parse_record_lifetime(row, 4, 5, 6)?;
                        Ok(Reader::create(
                            id,
                            full_name,
                            phone_number,
                            card_number,
                            record_lifetime,
                        ))
                    })
                    .optional()
            })
            .await;
        self.connection_pool.return_connection(client).await?;
        let reader = result.map_err(storage_err)?;
        Ok(reader)
    }

    async fn update(&self, reader: &Reader) -> StorageResult<()> {
        const QUERY: &str = "UPDATE readers SET full_name = ?1, phone_number = ?2, card_number = ?3, updated_at = ?4 WHERE id = ?5 AND deleted_at IS NULL";

        let full_name = reader.full_name.clone();
        let phone_number = reader.phone_number.clone();
        let card_number = reader.card_number.clone();
        let id = reader.id;
        let updated_at = encode_datetime(UtcDateTime::now());

        let client = self.connection_pool.acquire_connection().await?;
        let result = client
            .conn(move |connection| {
                let parameters: Vec<Value> = vec![
                    full_name.into(),
                    phone_number.into(),
                    card_number.into(),
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

    async fn remove(&self, reader: Reader) -> StorageResult<()> {
        const QUERY: &str =
            "UPDATE readers SET deleted_at = ?1 WHERE id = ?2 AND deleted_at IS NULL";

        let id = reader.id;

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
