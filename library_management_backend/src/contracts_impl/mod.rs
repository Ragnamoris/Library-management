pub mod books;
pub mod converter;
pub mod loans;
pub mod managed_connection_pool;
pub mod migrations;
pub mod publishers;
pub mod readers;

pub use books::BooksSqliteStorage;
pub use loans::LoansSqliteStorage;
pub use managed_connection_pool::ManagedConnectionPool;
pub use publishers::PublishersSqliteStorage;
pub use readers::ReadersSqliteStorage;

use crate::contracts::StorageError;

pub(crate) fn storage_err<E: std::fmt::Display>(e: E) -> StorageError {
    e.to_string().into()
}