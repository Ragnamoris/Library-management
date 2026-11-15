#[derive(Debug, Clone)]
pub struct Name(String);

#[derive(Debug, thiserror::Error)]
pub enum NameError {
    #[error("name is empty")]
    IsEmpty,
    #[error("name is too short")]
    LessThanMinLength,
    #[error("name is too long")]
    ExceedsMaxLength,
}

impl Name {
    pub fn new(value: impl Into<String>) -> Result<Self, NameError> {
        let value = value.into();
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(NameError::IsEmpty);
        }
        if trimmed.len() < 2 {
            return Err(NameError::LessThanMinLength);
        }
        if trimmed.len() > 255 {
            return Err(NameError::ExceedsMaxLength);
        }

        Ok(Self(trimmed.to_string()))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
