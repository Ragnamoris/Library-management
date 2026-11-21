use crate::entities::errors::DomainError;

/// Типизированные ошибки слоя хранения данных.
#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("Запись не найдена")]
    NotFound,
    #[error("Нарушение уникальности данных: {0}")]
    UniqueViolation(String),
    #[error("Нарушение ссылочной целостности: {0}")]
    ForeignKeyViolation(String),
    #[error("Нарушение ограничения CHECK: {0}")]
    CheckViolation(String),
    #[error("Не заполнено обязательное поле: {0}")]
    NotNullViolation(String),
    #[error("Нарушение правила предметной области: {0}")]
    BusinessRule(String),
    #[error("Ошибка преобразования данных: {0}")]
    Conversion(String),
    #[error("Ошибка базы данных: {0}")]
    Database(String),
    #[error("Ошибка пула соединений: {0}")]
    Pool(String),
}

impl From<StorageError> for DomainError {
    fn from(error: StorageError) -> Self {
        match error {
            StorageError::NotFound => DomainError::not_found("Запись"),
            StorageError::UniqueViolation(message) => {
                DomainError::conflict(format!("Такая запись уже существует: {message}"))
            }
            StorageError::ForeignKeyViolation(message) => {
                DomainError::validation(format!("Ссылка на несуществующую запись: {message}"))
            }
            StorageError::CheckViolation(message) | StorageError::NotNullViolation(message) => {
                DomainError::validation(message)
            }
            StorageError::BusinessRule(message) => DomainError::conflict(message),
            StorageError::Conversion(message)
            | StorageError::Database(message)
            | StorageError::Pool(message) => DomainError::storage(message),
        }
    }
}