#[derive(Debug, Clone)]
pub struct Address(String);

#[derive(Debug, thiserror::Error)]
pub enum AddressError {
    #[error("address is empty")]
    AddressIsEmpty,
    #[error("address is too long")]
    AddressTooLong,
}

impl Address {
    pub fn new(value: impl Into<String>) -> Result<Self, AddressError> {
        let value = value.into();
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(AddressError::AddressIsEmpty);
        }
        if trimmed.len() > 500 {
            return Err(AddressError::AddressTooLong);
        }

        Ok(Self(trimmed.to_string()))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
