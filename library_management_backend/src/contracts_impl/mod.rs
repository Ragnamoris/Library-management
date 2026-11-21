pub mod books;
pub mod converter;
pub mod errors;
pub mod loans;
pub mod managed_connection_pool;
pub mod migrations;
pub mod publishers;
pub mod query;
pub mod readers;
pub mod staff;

#[cfg(test)]
mod tests;

pub use books::BooksSqliteStorage;
pub use loans::LoansSqliteStorage;
pub use managed_connection_pool::ManagedConnectionPool;
pub use publishers::PublishersSqliteStorage;
pub use readers::ReadersSqliteStorage;
pub use staff::StaffSqliteStorage;

pub(crate) use errors::storage_err;