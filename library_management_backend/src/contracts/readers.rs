use std::future::Future;

use crate::contracts::specifications::ReaderSpecification;
use crate::contracts::StorageResult;
use crate::entities::Reader;

pub trait ReadersStorage {
    fn add(&self, reader: Reader) -> impl Future<Output = StorageResult<()>>;
    fn get(
        &self,
        specification: ReaderSpecification,
    ) -> impl Future<Output = StorageResult<Option<Reader>>>;
    fn update(&self, reader: &Reader) -> impl Future<Output = StorageResult<()>>;
    fn remove(&self, reader: Reader) -> impl Future<Output = StorageResult<()>>;
}
