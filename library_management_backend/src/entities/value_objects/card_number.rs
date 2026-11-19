#[derive(Debug, Clone)]
pub struct CardNumber(String);

#[derive(Debug, thiserror::Error)]
pub enum CardNumberError {
    #[error("card number is empty")]
    NumberIsEmpty,
    #[error("card number is too long")]
    NumberTooLong,
}

impl CardNumber {
    pub fn new(value: impl Into<String>) -> Result<Self, CardNumberError> {
        let value = value.into();
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(CardNumberError::NumberIsEmpty);
        }
        if trimmed.len() > 64 {
            return Err(CardNumberError::NumberTooLong);
        }

        Ok(Self(trimmed.to_string()))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
