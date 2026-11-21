pub mod books;
pub mod errors;
pub mod loans;
pub mod publishers;
pub mod readers;
pub mod specifications;
pub mod staff;

pub use books::BooksStorage;
pub use errors::StorageError;
pub use loans::LoansStorage;
pub use publishers::PublishersStorage;
pub use readers::ReadersStorage;
pub use specifications::{
    BookSpecification, LoanSpecification, PublisherSpecification, ReaderSpecification,
    StaffSpecification,
};
pub use staff::StaffStorage;

pub type StorageResult<T> = Result<T, StorageError>;