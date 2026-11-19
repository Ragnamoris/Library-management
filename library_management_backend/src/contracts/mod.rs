pub mod books;
pub mod loans;
pub mod publishers;
pub mod readers;
pub mod specifications;

pub use books::BooksStorage;
pub use loans::LoansStorage;
pub use publishers::PublishersStorage;
pub use readers::ReadersStorage;
pub use specifications::PublisherSpecification;

pub type StorageError = Box<dyn std::error::Error + Send + Sync>;
pub type StorageResult<T> = Result<T, StorageError>;
