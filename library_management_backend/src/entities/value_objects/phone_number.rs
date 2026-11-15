use std::sync::LazyLock;

use regex::Regex;

static PHONE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\A7[0-9]{10}\z").expect("phone regex must be valid"));

#[derive(Debug, Clone)]
pub struct PhoneNumber(String);

#[derive(Debug, thiserror::Error)]
pub enum PhoneNumberError {
    #[error("phone number is empty")]
    EmptyNumber,
    #[error("phone number format is invalid")]
    InvalidFormat,
}

impl PhoneNumber {
    pub fn new(value: impl Into<String>) -> Result<Self, PhoneNumberError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(PhoneNumberError::EmptyNumber);
        }
        if !PHONE_REGEX.is_match(&value) {
            return Err(PhoneNumberError::InvalidFormat);
        }

        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
