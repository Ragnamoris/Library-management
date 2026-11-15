use std::sync::Arc;

use rusqlite::{params_from_iter, types::Value, OptionalExtension};
use time::UtcDateTime;

use crate::contracts::specifications::BookSpecification;
use crate::contracts::{BooksStorage, StorageResult};
use crate::contracts_impl::converter::parse_record_lifetime;
use crate::contracts_impl::{storage_err, ManagedConnectionPool};
use crate::entities::value_objects::{EntityId, InventoryNumber, Name, PublicationYear};
use crate::entities::Book;
use crate::utils::{encode_datetime, encode_optional_datetime};

pub struct BooksSqliteStorage {
    connection_pool: Arc<ManagedConnectionPool>,
}

impl BooksSqliteStorage {
    pub fn new(connection_pool: Arc<ManagedConnectionPool>) -> Self {
        Self { connection_pool }
    }
}

impl BooksStorage for BooksSqliteStorage {
    async fn add(&self, book: Book) -> StorageResult<()> {
        const QUERY: &str = "INSERT INTO books(id, publisher_id, title, author, publication_year, inventory_number, created_at, updated_at, deleted_at) VALUES(?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)";

        let client = self.connection_pool.acquire_connection().await?;
        let result = client
            .conn(move |connection| {
                let parameters: Vec<Value> = vec![
                    book.id.into(),
                    book.publisher_id.into(),
                    book.title.into(),
                    book.author.into(),
                    book.publication_year.into(),
                    book.inventory_number.into(),
                    encode_datetime(book.record_lifetime.created_at()).into(),
                    encode_optional_datetime(book.record_lifetime.updated_at()).into(),
                    encode_optional_datetime(book.record_lifetime.deleted_at()).into(),
                ];
                connection.execute(QUERY, params_from_iter(parameters))
            })
            .await;
        self.connection_pool.return_connection(client).await?;
        result.map_err(storage_err)?;
        Ok(())
    }

    async fn get(&self, specification: BookSpecification) -> StorageResult<Option<Book>> {
        let mut filters: Vec<&str> = Vec::new();
        let mut parameters: Vec<Value> = Vec::new();

        if let Some(id) = specification.id {
            filters.push("id = ?");
            parameters.push(id.into());
        }
        if let Some(publisher_id) = specification.publisher_id {
            filters.push("publisher_id = ?");
            parameters.push(publisher_id.into());
        }
        if let Some(title) = specification.title {
            filters.push("title = ?");
            parameters.push(title.into());
        }
        if let Some(author) = specification.author {
            filters.push("author = ?");
            parameters.push(author.into());
        }
        if let Some(inventory_number) = specification.inventory_number {
            filters.push("inventory_number = ?");
            parameters.push(inventory_number.into());
        }

        if filters.is_empty() {
            return Ok(None);
        }

        let query: String = format!(
            "SELECT id, publisher_id, title, author, publication_year, inventory_number, created_at, updated_at, deleted_at FROM books WHERE deleted_at IS NULL AND {}",
            filters.join(" AND ")
        );

        let client = self.connection_pool.acquire_connection().await?;
        let result = client
            .conn(move |connection| {
                let mut statement = connection.prepare(&query)?;
                statement
                    .query_row(params_from_iter(parameters), |row| {
                        let id: EntityId = row.get(0)?;
                        let publisher_id: EntityId = row.get(1)?;
                        let title: Name = row.get(2)?;
                        let author: Name = row.get(3)?;
                        let publication_year: PublicationYear = row.get(4)?;
                        let inventory_number: InventoryNumber = row.get(5)?;
                        let record_lifetime = parse_record_lifetime(row, 6, 7, 8)?;
                        Ok(Book::create(
                            id,
                            publisher_id,
                            title,
                            author,
                            publication_year,
                            inventory_number,
                            record_lifetime,
                        ))
                    })
                    .optional()
            })
            .await;
        self.connection_pool.return_connection(client).await?;
        let book = result.map_err(storage_err)?;
        Ok(book)
    }

    async fn update(&self, book: &Book) -> StorageResult<()> {
        const QUERY: &str = "UPDATE books SET publisher_id = ?1, title = ?2, author = ?3, publication_year = ?4, inventory_number = ?5, updated_at = ?6 WHERE id = ?7 AND deleted_at IS NULL";

        let publisher_id = book.publisher_id;
        let title = book.title.clone();
        let author = book.author.clone();
        let publication_year = book.publication_year;
        let inventory_number = book.inventory_number.clone();
        let id = book.id;
        let updated_at = encode_datetime(UtcDateTime::now());

        let client = self.connection_pool.acquire_connection().await?;
        let result = client
            .conn(move |connection| {
                let parameters: Vec<Value> = vec![
                    publisher_id.into(),
                    title.into(),
                    author.into(),
                    publication_year.into(),
                    inventory_number.into(),
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

    async fn remove(&self, book: Book) -> StorageResult<()> {
        const QUERY: &str = "UPDATE books SET deleted_at = ?1 WHERE id = ?2 AND deleted_at IS NULL";

        let id = book.id;

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
