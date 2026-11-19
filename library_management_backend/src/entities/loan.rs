use time::Duration;
use time::UtcDateTime;

use crate::entities::value_objects::{
    EntityId, EntityIdError, RecordLifetime, RecordLifetimeError,
};
use crate::entities::{Book, Reader};

pub const LOAN_TERM_DAYS: i64 = 20;
pub const MAX_ACTIVE_LOANS: i64 = 5;

#[doc = "Выдача книги читателю"]
#[derive(Debug, Clone)]
pub struct Loan {
    pub(crate) id: EntityId,
    pub(crate) book_id: EntityId,
    pub(crate) reader_id: EntityId,
    pub(crate) issued_at: UtcDateTime,
    pub(crate) due_at: UtcDateTime,
    pub(crate) returned_at: Option<UtcDateTime>,
    pub(crate) record_lifetime: RecordLifetime,
}

#[derive(Debug, thiserror::Error)]
pub enum LoanError {
    #[error("loan id error: {0}")]
    Id(#[from] EntityIdError),
    #[error("loan record lifetime error: {0}")]
    RecordLifetime(#[from] RecordLifetimeError),
    #[error("due date is earlier than issue date")]
    DueBeforeIssued,
}

impl Loan {
    pub fn create_new(book: &Book, reader: &Reader) -> Result<Self, LoanError> {
        let issued_at: UtcDateTime = UtcDateTime::now();
        let due_at: UtcDateTime = issued_at + Duration::days(LOAN_TERM_DAYS);
        let record_lifetime = RecordLifetime::create(issued_at, None, None)?;

        Self::create(
            EntityId::new(),
            book.id,
            reader.id,
            issued_at,
            due_at,
            None,
            record_lifetime,
        )
    }

    pub fn create(
        id: EntityId,
        book_id: EntityId,
        reader_id: EntityId,
        issued_at: UtcDateTime,
        due_at: UtcDateTime,
        returned_at: Option<UtcDateTime>,
        record_lifetime: RecordLifetime,
    ) -> Result<Self, LoanError> {
        if due_at < issued_at {
            return Err(LoanError::DueBeforeIssued);
        }

        Ok(Self {
            id,
            book_id,
            reader_id,
            issued_at,
            due_at,
            returned_at,
            record_lifetime,
        })
    }

    pub fn is_returned(&self) -> bool {
        self.returned_at.is_some()
    }

    pub fn is_overdue(&self, at: UtcDateTime) -> bool {
        self.returned_at.is_none() && at > self.due_at
    }
}
