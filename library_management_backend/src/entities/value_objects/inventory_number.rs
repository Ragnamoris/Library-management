#[derive(Debug, Clone)]
pub struct InventoryNumber(String);

#[derive(Debug, thiserror::Error)]
pub enum InventoryNumberError {
    #[error("inventory number is empty")]
    NumberIsEmpty,
    #[error("inventory number is too long")]
    NumberTooLong,
}

impl InventoryNumber {
    pub fn new(value: impl Into<String>) -> Result<Self, InventoryNumberError> {
        let value = value.into();
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(InventoryNumberError::NumberIsEmpty);
        }
        if trimmed.len() > 64 {
            return Err(InventoryNumberError::NumberTooLong);
        }

        Ok(Self(trimmed.to_string()))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
