use std::future::Future;

use uuid::Uuid;

use crate::contracts::specifications::LoanSpecification;
use crate::contracts::StorageResult;
use crate::entities::Loan;

pub trait LoansStorage {
    fn add(&self, loan: Loan) -> impl Future<Output = StorageResult<()>>;
    fn get(
        &self,
        specification: LoanSpecification,
    ) -> impl Future<Output = StorageResult<Option<Loan>>>;
    fn update(&self, loan: &Loan) -> impl Future<Output = StorageResult<()>>;
    fn remove(&self, loan: Loan) -> impl Future<Output = StorageResult<()>>;
    fn count_active_for_reader(&self, reader_id: Uuid) -> impl Future<Output = StorageResult<i64>>;
    fn is_book_issued(&self, book_id: Uuid) -> impl Future<Output = StorageResult<bool>>;
}
