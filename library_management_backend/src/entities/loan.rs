use time::{Duration, UtcDateTime};

use crate::entities::errors::DomainError;
use crate::entities::value_objects::{
    EntityId, EntityIdError, RecordLifetime, RecordLifetimeError,
};
use crate::entities::{Book, Reader};

pub const LOAN_TERM_DAYS: i64 = 20;
pub const MAX_ACTIVE_LOANS: i64 = 5;

#[doc = "Состояние выдачи книги"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoanStatus {
    /// Книга на руках, срок не истёк.
    Active,
    /// Книга на руках, срок истёк.
    Overdue,
    /// Книга возвращена в срок.
    Returned,
    /// Книга возвращена с нарушением срока.
    ReturnedLate,
}

impl LoanStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            LoanStatus::Active => "active",
            LoanStatus::Overdue => "overdue",
            LoanStatus::Returned => "returned",
            LoanStatus::ReturnedLate => "returned_late",
        }
    }

    pub fn title(&self) -> &'static str {
        match self {
            LoanStatus::Active => "На руках",
            LoanStatus::Overdue => "Просрочена",
            LoanStatus::Returned => "Возвращена",
            LoanStatus::ReturnedLate => "Возвращена с просрочкой",
        }
    }
}

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
    #[error("плановая дата возврата раньше даты выдачи")]
    DueBeforeIssued,
    #[error("срок пользования книгой не может превышать 20 дней")]
    TermTooLong,
    #[error("дата возврата раньше даты выдачи")]
    ReturnBeforeIssued,
}

impl Loan {
    pub fn issue(
        book: &Book,
        reader: &Reader,
        active_loans_of_reader: i64,
        book_has_active_loan: bool,
        issued_at: UtcDateTime,
    ) -> Result<Self, DomainError> {
        if book_has_active_loan {
            return Err(DomainError::conflict(
                "Книга уже находится на руках и не может быть выдана повторно",
            ));
        }

        if active_loans_of_reader >= MAX_ACTIVE_LOANS {
            return Err(DomainError::conflict(format!(
                "У читателя уже {active_loans_of_reader} книг на руках — достигнут предел в {MAX_ACTIVE_LOANS} книг"
            )));
        }

        if active_loans_of_reader < 0 {
            return Err(DomainError::validation(
                "количество книг на руках не может быть отрицательным",
            ));
        }

        let due_at = issued_at + Duration::days(LOAN_TERM_DAYS);
        let record_lifetime = RecordLifetime::create(issued_at, None, None)?;

        let loan = Self::create(
            EntityId::new(),
            book.id,
            reader.id,
            issued_at,
            due_at,
            None,
            record_lifetime,
        )?;

        Ok(loan)
    }

    /// Низкоуровневый конструктор: используется адаптером БД и демонстрационными данными.
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
        if due_at - issued_at > Duration::days(LOAN_TERM_DAYS) {
            return Err(LoanError::TermTooLong);
        }
        if let Some(returned_at) = returned_at {
            if returned_at < issued_at {
                return Err(LoanError::ReturnBeforeIssued);
            }
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

    /// Приём возврата книги. Повторное закрытие выдачи не допускается.
    pub fn close(&mut self, returned_at: UtcDateTime) -> Result<(), DomainError> {
        if self.is_returned() {
            return Err(DomainError::conflict(
                "Выдача уже закрыта: книга была возвращена ранее",
            ));
        }
        if returned_at < self.issued_at {
            return Err(DomainError::validation(
                "дата возврата не может быть раньше даты выдачи",
            ));
        }

        self.returned_at = Some(returned_at);
        self.record_lifetime = RecordLifetime::create(
            self.record_lifetime.created_at(),
            Some(returned_at),
            self.record_lifetime.deleted_at(),
        )?;
        Ok(())
    }

    pub fn id(&self) -> EntityId {
        self.id
    }

    pub fn book_id(&self) -> EntityId {
        self.book_id
    }

    pub fn reader_id(&self) -> EntityId {
        self.reader_id
    }

    pub fn issued_at(&self) -> UtcDateTime {
        self.issued_at
    }

    pub fn due_at(&self) -> UtcDateTime {
        self.due_at
    }

    pub fn returned_at(&self) -> Option<UtcDateTime> {
        self.returned_at
    }

    pub fn record_lifetime(&self) -> &RecordLifetime {
        &self.record_lifetime
    }

    pub fn is_returned(&self) -> bool {
        self.returned_at.is_some()
    }

    pub fn is_active(&self) -> bool {
        self.returned_at.is_none()
    }

    pub fn is_overdue(&self, at: UtcDateTime) -> bool {
        self.returned_at.is_none() && at > self.due_at
    }

    /// Количество дней просрочки (0, если просрочки нет).
    pub fn overdue_days(&self, at: UtcDateTime) -> i64 {
        let border = self.returned_at.unwrap_or(at);
        if border <= self.due_at {
            return 0;
        }
        (border - self.due_at).whole_days()
    }

    pub fn status(&self, at: UtcDateTime) -> LoanStatus {
        match self.returned_at {
            Some(returned_at) if returned_at > self.due_at => LoanStatus::ReturnedLate,
            Some(_) => LoanStatus::Returned,
            None if at > self.due_at => LoanStatus::Overdue,
            None => LoanStatus::Active,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::value_objects::{
        Address, CardNumber, InventoryNumber, Name, PhoneNumber, PublicationYear,
    };
    use crate::entities::Publisher;

    fn sample_book() -> Book {
        let publisher = Publisher::create_new(
            Name::new("АСТ".to_string()).unwrap(),
            Address::new("г. Москва, ул. Кирова, д. 12".to_string()).unwrap(),
        )
        .unwrap();

        Book::create_new(
            &publisher,
            Name::new("Мастер и Маргарита".to_string()).unwrap(),
            Name::new("М. А. Булгаков".to_string()).unwrap(),
            PublicationYear::new(2019).unwrap(),
            InventoryNumber::new("ИНВ-0001".to_string()).unwrap(),
        )
        .unwrap()
    }

        fn sample_phone() -> PhoneNumber {
        PhoneNumber::new("79001112233").unwrap()
    }

    fn sample_card_number() -> CardNumber {
        CardNumber::new("ЧБ-0001").unwrap()
    }

    fn sample_reader() -> Reader {
        Reader::create_new(
            Name::new("Иванов Иван Иванович".to_string()).unwrap(),
            sample_phone(),
            sample_card_number(),
        )
        .unwrap()
    }
    fn issued_loan() -> Loan {
        Loan::issue(
            &sample_book(),
            &sample_reader(),
            0,
            false,
            UtcDateTime::now(),
        )
        .unwrap()
    }

    #[test]
    fn issue_sets_twenty_day_term() {
        let loan = issued_loan();
        assert_eq!((loan.due_at() - loan.issued_at()).whole_days(), LOAN_TERM_DAYS);
        assert!(loan.is_active());
    }

    #[test]
    fn issue_rejects_book_that_is_on_hands() {
        let result = Loan::issue(&sample_book(), &sample_reader(), 0, true, UtcDateTime::now());
        assert!(matches!(result, Err(DomainError::Conflict(_))));
    }

    #[test]
    fn issue_rejects_when_reader_reached_limit() {
        let result = Loan::issue(
            &sample_book(),
            &sample_reader(),
            MAX_ACTIVE_LOANS,
            false,
            UtcDateTime::now(),
        );
        assert!(matches!(result, Err(DomainError::Conflict(_))));
    }

    #[test]
    fn issue_allows_four_books_on_hands() {
        let result = Loan::issue(
            &sample_book(),
            &sample_reader(),
            MAX_ACTIVE_LOANS - 1,
            false,
            UtcDateTime::now(),
        );
        assert!(result.is_ok());
    }

    #[test]
    fn close_marks_loan_as_returned() {
        let mut loan = issued_loan();
        let returned_at = loan.issued_at() + Duration::days(5);
        loan.close(returned_at).unwrap();

        assert!(loan.is_returned());
        assert_eq!(loan.status(UtcDateTime::now()), LoanStatus::Returned);
        assert_eq!(loan.overdue_days(UtcDateTime::now()), 0);
    }

    #[test]
    fn second_close_is_rejected() {
        let mut loan = issued_loan();
        let returned_at = loan.issued_at() + Duration::days(5);
        loan.close(returned_at).unwrap();

        let result = loan.close(returned_at + Duration::days(1));
        assert!(matches!(result, Err(DomainError::Conflict(_))));
    }

    #[test]
    fn return_before_issue_is_rejected() {
        let mut loan = issued_loan();
        let result = loan.close(loan.issued_at() - Duration::days(1));
        assert!(matches!(result, Err(DomainError::Validation(_))));
    }

    #[test]
    fn overdue_is_detected_after_term() {
        let loan = issued_loan();
        let now = loan.due_at() + Duration::days(3);

        assert!(loan.is_overdue(now));
        assert_eq!(loan.status(now), LoanStatus::Overdue);
        assert_eq!(loan.overdue_days(now), 3);
    }

    #[test]
    fn late_return_is_marked_separately() {
        let mut loan = issued_loan();
        let returned_at = loan.due_at() + Duration::days(4);
        loan.close(returned_at).unwrap();

        assert_eq!(loan.status(UtcDateTime::now()), LoanStatus::ReturnedLate);
        assert_eq!(loan.overdue_days(UtcDateTime::now()), 4);
        assert!(!loan.is_overdue(UtcDateTime::now()));
    }

    #[test]
    fn term_longer_than_twenty_days_is_rejected() {
        let issued_at = UtcDateTime::now();
        let result = Loan::create(
            EntityId::new(),
            EntityId::new(),
            EntityId::new(),
            issued_at,
            issued_at + Duration::days(LOAN_TERM_DAYS + 1),
            None,
            RecordLifetime::create(issued_at, None, None).unwrap(),
        );
        assert!(matches!(result, Err(LoanError::TermTooLong)));
    }
}