use std::future::Future;

use crate::contracts::specifications::StaffSpecification;
use crate::contracts::StorageResult;
use crate::entities::Staff;

pub trait StaffStorage {
    fn add(&self, staff: Staff) -> impl Future<Output = StorageResult<()>>;
    fn get(
        &self,
        specification: StaffSpecification,
    ) -> impl Future<Output = StorageResult<Option<Staff>>>;
    fn list(
        &self,
        specification: StaffSpecification,
    ) -> impl Future<Output = StorageResult<Vec<Staff>>>;
    fn update(&self, staff: &Staff) -> impl Future<Output = StorageResult<()>>;
    fn remove(&self, staff: Staff) -> impl Future<Output = StorageResult<()>>;
}