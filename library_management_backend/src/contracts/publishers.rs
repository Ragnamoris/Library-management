use std::future::Future;

use crate::contracts::specifications::PublisherSpecification;
use crate::contracts::StorageResult;
use crate::entities::Publisher;

pub trait PublishersStorage {
    fn add(&self, publisher: Publisher) -> impl Future<Output = StorageResult<()>>;
    fn get(
        &self,
        specification: PublisherSpecification,
    ) -> impl Future<Output = StorageResult<Option<Publisher>>>;
    fn list(
        &self,
        specification: PublisherSpecification,
    ) -> impl Future<Output = StorageResult<Vec<Publisher>>>;
    fn count(&self, specification: PublisherSpecification)
        -> impl Future<Output = StorageResult<i64>>;
    fn update(&self, publisher: &Publisher) -> impl Future<Output = StorageResult<()>>;
    fn remove(&self, publisher: Publisher) -> impl Future<Output = StorageResult<()>>;
}