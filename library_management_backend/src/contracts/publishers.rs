use std::future::Future;

use crate::contracts::{PublisherSpecification, StorageResult};
use crate::entities::Publisher;

pub trait PublishersStorage {
    fn add(&self, publisher: Publisher) -> impl Future<Output = StorageResult<()>>;
    fn get(
        &self,
        specification: PublisherSpecification,
    ) -> impl Future<Output = StorageResult<Option<Publisher>>>;
    fn update(&self, publisher: &Publisher) -> impl Future<Output = StorageResult<()>>;
    fn remove(&self, publisher: Publisher) -> impl Future<Output = StorageResult<()>>;
}
