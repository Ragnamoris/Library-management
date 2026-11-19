use crate::entities::book::BookError;
use crate::entities::loan::LoanError;
use crate::entities::publisher::PublisherError;
use crate::entities::reader::ReaderError;
use crate::entities::staff::StaffError;
use crate::entities::value_objects::{
    AddressError, CardNumberError, EntityIdError, LoginError, NameError, PasswordHashError,
    PhoneNumberError, RecordLifetimeError, StaffRoleError,
};

#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    /// Некорректные входные данные или нарушение инварианта объекта-значения.
    #[error("Некорректные данные: {0}")]
    Validation(String),

    /// Сущность, необходимая для операции, не найдена.
    #[error("{0}")]
    NotFound(String),

    /// Операция запрещена текущим состоянием предметной области.
    #[error("Нарушение правил предметной области: {0}")]
    Conflict(String),

    /// Операция требует авторизации.
    #[error("Требуется авторизация")]
    Unauthorized,

    /// Роль работника не позволяет выполнить операцию.
    #[error("Недостаточно прав доступа для выполнения операции")]
    Forbidden,

    /// Ошибка уровня хранения данных.
    #[error("Ошибка хранилища данных: {0}")]
    Storage(String),
}

impl DomainError {
    /// Оборачивает любую ошибку валидации в доменную ошибку.
    pub fn validation<E: std::fmt::Display>(error: E) -> Self {
        DomainError::Validation(error.to_string())
    }

    pub fn not_found(entity: &str) -> Self {
        DomainError::NotFound(format!("{entity} не найден(а)"))
    }

    pub fn conflict(message: impl Into<String>) -> Self {
        DomainError::Conflict(message.into())
    }

    pub fn storage<E: std::fmt::Display>(error: E) -> Self {
        DomainError::Storage(error.to_string())
    }
}

pub type DomainResult<T> = Result<T, DomainError>;

macro_rules! validation_error_from {
    ($($error_type:ty),+ $(,)?) => {
        $(
            impl From<$error_type> for DomainError {
                fn from(error: $error_type) -> Self {
                    DomainError::Validation(error.to_string())
                }
            }
        )+
    };
}

validation_error_from!(
    EntityIdError,
    RecordLifetimeError,
    NameError,
    AddressError,
    PhoneNumberError,
    CardNumberError,
    LoginError,
    PasswordHashError,
    StaffRoleError,
    BookError,
    PublisherError,
    ReaderError,
    StaffError,
    LoanError,
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validation_error_keeps_message() {
        let error = DomainError::validation("инвентарный номер пуст");
        assert!(matches!(error, DomainError::Validation(_)));
        assert!(error.to_string().contains("инвентарный номер пуст"));
    }

    #[test]
    fn not_found_error_mentions_entity() {
        let error = DomainError::not_found("Книга");
        assert!(matches!(error, DomainError::NotFound(_)));
        assert!(error.to_string().contains("Книга"));
    }
}