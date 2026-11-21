use std::future::Future;

use crate::contracts::specifications::BookSpecification;
use crate::contracts::StorageResult;
use crate::entities::Book;

pub trait BooksStorage {
    fn add(&self, book: Book) -> impl Future<Output = StorageResult<()>>;
    fn get(
        &self,
        specification: BookSpecification,
    ) -> impl Future<Output = StorageResult<Option<Book>>>;
    fn list(
        &self,
        specification: BookSpecification,
    ) -> impl Future<Output = StorageResult<Vec<Book>>>;
    fn count(&self, specification: BookSpecification) -> impl Future<Output = StorageResult<i64>>;
    fn update(&self, book: &Book) -> impl Future<Output = StorageResult<()>>;
    fn remove(&self, book: Book) -> impl Future<Output = StorageResult<()>>;
}