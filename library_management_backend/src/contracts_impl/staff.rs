use std::sync::Arc;

use rusqlite::{params_from_iter, types::Value, OptionalExtension};
use time::UtcDateTime;

use crate::contracts::specifications::StaffSpecification;
use crate::contracts::{StaffStorage, StorageResult};
use crate::contracts_impl::converter::parse_record_lifetime;
use crate::contracts_impl::query::QueryFilter;
use crate::contracts_impl::{storage_err, ManagedConnectionPool};
use crate::entities::value_objects::{EntityId, Login, Name, PasswordHash, StaffRole};
use crate::entities::Staff;
use crate::utils::{encode_datetime, encode_optional_datetime};

const SELECT: &str = "SELECT id, login, password_hash, full_name, role, created_at, updated_at, deleted_at FROM staff";

pub struct StaffSqliteStorage {
    connection_pool: Arc<ManagedConnectionPool>,
}

impl StaffSqliteStorage {
    pub fn new(connection_pool: Arc<ManagedConnectionPool>) -> Self {
        Self { connection_pool }
    }

    fn filter(specification: &StaffSpecification) -> QueryFilter {
        let mut filter = QueryFilter::new();
        if let Some(id) = specification.id {
            filter.eq("id", id);
        }
        if let Some(login) = &specification.login {
            filter.eq("login", login.value().to_string());
        }
        if let Some(role) = specification.role {
        filter.eq("role", role.as_str().to_string());
        }
        filter
    }
}

fn map_row(row: &rusqlite::Row) -> rusqlite::Result<Staff> {
    let id: EntityId = row.get(0)?;
    let login: Login = row.get(1)?;
    let password_hash: PasswordHash = row.get(2)?;
    let full_name: Name = row.get(3)?;
    let role: StaffRole = row.get(4)?;
    let record_lifetime = parse_record_lifetime(row, 5, 6, 7)?;
    Ok(Staff::create(id, login, password_hash, full_name, role, record_lifetime))
}

impl StaffStorage for StaffSqliteStorage {
    async fn add(&self, staff: Staff) -> StorageResult<()> {
        let parameters: Vec<Value> = vec![
            staff.id.into(),
            staff.login.clone().into(),
            staff.password_hash.clone().into(),
            staff.full_name.clone().into(),
            staff.role.into(),
            encode_datetime(staff.record_lifetime.created_at()).into(),
            encode_optional_datetime(staff.record_lifetime.updated_at()).into(),
            encode_optional_datetime(staff.record_lifetime.deleted_at()).into(),
        ];

        let client = self.connection_pool.acquire_connection().await?;
        let result = client
            .conn(move |connection| {
                connection.execute(
                    "INSERT INTO staff(id, login, password_hash, full_name, role, created_at, updated_at, deleted_at)
                     VALUES(?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                    params_from_iter(parameters),
                )
            })
            .await;
        self.connection_pool.return_connection(client).await?;
        result.map_err(storage_err)?;
        Ok(())
    }

    async fn get(&self, specification: StaffSpecification) -> StorageResult<Option<Staff>> {
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

    async fn list(&self, specification: StaffSpecification) -> StorageResult<Vec<Staff>> {
        let filter = Self::filter(&specification);
        let query = format!(
            "{SELECT}{}{}",
            filter.where_clause(),
            filter.suffix("login", specification.limit, specification.offset)
        );
        let parameters = filter.parameters();

        let client = self.connection_pool.acquire_connection().await?;
        let result = client
            .conn(move |connection| {
                let mut statement = connection.prepare(&query)?;
                let rows = statement.query_map(params_from_iter(parameters), map_row)?;
                rows.collect::<rusqlite::Result<Vec<Staff>>>()
            })
            .await;
        self.connection_pool.return_connection(client).await?;
        result.map_err(storage_err)
    }

    async fn update(&self, staff: &Staff) -> StorageResult<()> {
        let parameters: Vec<Value> = vec![
            staff.login.clone().into(),
            staff.password_hash.clone().into(),
            staff.full_name.clone().into(),
            staff.role.into(),
            encode_datetime(UtcDateTime::now()).into(),
            staff.id.into(),
        ];

        let client = self.connection_pool.acquire_connection().await?;
        let result = client
            .conn(move |connection| {
                connection.execute(
                    "UPDATE staff SET login = ?1, password_hash = ?2, full_name = ?3, role = ?4, updated_at = ?5
                     WHERE id = ?6 AND deleted_at IS NULL",
                    params_from_iter(parameters),
                )
            })
            .await;
        self.connection_pool.return_connection(client).await?;
        result.map_err(storage_err)?;
        Ok(())
    }

    async fn remove(&self, staff: Staff) -> StorageResult<()> {
        let parameters: Vec<Value> = vec![encode_datetime(UtcDateTime::now()).into(), staff.id.into()];

        let client = self.connection_pool.acquire_connection().await?;
        let result = client
            .conn(move |connection| {
                connection.execute(
                    "UPDATE staff SET deleted_at = ?1 WHERE id = ?2 AND deleted_at IS NULL",
                    params_from_iter(parameters),
                )
            })
            .await;
        self.connection_pool.return_connection(client).await?;
        result.map_err(storage_err)?;
        Ok(())
    }
}