use std::sync::Arc;

use rusqlite::{params_from_iter, types::Value, OptionalExtension};
use time::UtcDateTime;

use crate::contracts::specifications::BookSpecification;
use crate::contracts::{BooksStorage, StorageResult};
use crate::contracts_impl::converter::parse_record_lifetime;
use crate::contracts_impl::query::QueryFilter;
use crate::contracts_impl::{storage_err, ManagedConnectionPool};
use crate::entities::value_objects::{EntityId, InventoryNumber, Name, PublicationYear};
use crate::entities::Book;
use crate::utils::{encode_datetime, encode_optional_datetime};

const SELECT: &str = "SELECT id, publisher_id, title, author, publication_year, inventory_number, created_at, updated_at, deleted_at FROM books";

pub struct BooksSqliteStorage {
    connection_pool: Arc<ManagedConnectionPool>,
}

impl BooksSqliteStorage {
    pub fn new(connection_pool: Arc<ManagedConnectionPool>) -> Self {
        Self { connection_pool }
    }

    fn filter(specification: &BookSpecification) -> QueryFilter {
        let mut filter = QueryFilter::new();
        if let Some(id) = specification.id {
            filter.eq("id", id);
        }
        if let Some(publisher_id) = specification.publisher_id {
            filter.eq("publisher_id", publisher_id);
        }
        if let Some(title) = &specification.title {
            filter.eq("title", title.value().to_string());
        }
        if let Some(author) = &specification.author {
            filter.eq("author", author.value().to_string());
        }
        if let Some(inventory_number) = &specification.inventory_number {
            filter.eq("inventory_number", inventory_number.value().to_string());
        }
        if let Some(search) = &specification.search {
            filter.search(&["title", "author", "inventory_number"], search);
        }
        if specification.only_available {
            filter.raw(
                "NOT EXISTS (SELECT 1 FROM loans l \
                 WHERE l.book_id = books.id AND l.returned_at IS NULL AND l.deleted_at IS NULL)",
            );
        }
        filter
    }
}

fn map_row(row: &rusqlite::Row) -> rusqlite::Result<Book> {
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
}

impl BooksStorage for BooksSqliteStorage {
    async fn add(&self, book: Book) -> StorageResult<()> {
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

        let client = self.connection_pool.acquire_connection().await?;
        let result = client
            .conn(move |connection| {
                connection.execute(
                    "INSERT INTO books(id, publisher_id, title, author, publication_year, inventory_number, created_at, updated_at, deleted_at)
                     VALUES(?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                    params_from_iter(parameters),
                )
            })
            .await;
        self.connection_pool.return_connection(client).await?;
        result.map_err(storage_err)?;
        Ok(())
    }

    async fn get(&self, specification: BookSpecification) -> StorageResult<Option<Book>> {
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

    async fn list(&self, specification: BookSpecification) -> StorageResult<Vec<Book>> {
        let filter = Self::filter(&specification);
        let query = format!(
            "{SELECT}{}{}",
            filter.where_clause(),
            filter.suffix("title", specification.limit, specification.offset)
        );
        let parameters = filter.parameters();

        let client = self.connection_pool.acquire_connection().await?;
        let result = client
            .conn(move |connection| {
                let mut statement = connection.prepare(&query)?;
                let rows = statement.query_map(params_from_iter(parameters), map_row)?;
                rows.collect::<rusqlite::Result<Vec<Book>>>()
            })
            .await;
        self.connection_pool.return_connection(client).await?;
        result.map_err(storage_err)
    }

    async fn count(&self, specification: BookSpecification) -> StorageResult<i64> {
        let filter = Self::filter(&specification);
        let query = format!("SELECT COUNT(*) FROM books{}", filter.where_clause());
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

    async fn update(&self, book: &Book) -> StorageResult<()> {
        let parameters: Vec<Value> = vec![
            book.publisher_id.into(),
            book.title.clone().into(),
            book.author.clone().into(),
            book.publication_year.into(),
            book.inventory_number.clone().into(),
            encode_datetime(UtcDateTime::now()).into(),
            book.id.into(),
        ];

        let client = self.connection_pool.acquire_connection().await?;
        let result = client
            .conn(move |connection| {
                connection.execute(
                    "UPDATE books SET publisher_id = ?1, title = ?2, author = ?3, publication_year = ?4,
                                      inventory_number = ?5, updated_at = ?6
                     WHERE id = ?7 AND deleted_at IS NULL",
                    params_from_iter(parameters),
                )
            })
            .await;
        self.connection_pool.return_connection(client).await?;
        result.map_err(storage_err)?;
        Ok(())
    }

    async fn remove(&self, book: Book) -> StorageResult<()> {
        let parameters: Vec<Value> = vec![encode_datetime(UtcDateTime::now()).into(), book.id.into()];

        let client = self.connection_pool.acquire_connection().await?;
        let result = client
            .conn(move |connection| {
                connection.execute(
                    "UPDATE books SET deleted_at = ?1 WHERE id = ?2 AND deleted_at IS NULL",
                    params_from_iter(parameters),
                )
            })
            .await;
        self.connection_pool.return_connection(client).await?;
        result.map_err(storage_err)?;
        Ok(())
    }
}