/// Хеш пароля работника. Открытый пароль в системе не хранится никогда.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PasswordHash(String);

#[derive(Debug, thiserror::Error)]
pub enum PasswordHashError {
    #[error("пароль должен содержать не менее 6 символов")]
    PasswordTooShort,
    #[error("хеш пароля повреждён или имеет неизвестный формат")]
    InvalidHash,
    #[error("не удалось вычислить хеш пароля: {0}")]
    HashingFailed(String),
}

impl PasswordHash {
    pub const MIN_PASSWORD_LENGTH: usize = 6;

    /// Хеширует пароль с стоимостью по умолчанию (используется при регистрации).
    pub fn from_password(password: &str) -> Result<Self, PasswordHashError> {
        Self::from_password_with_cost(password, bcrypt::DEFAULT_COST)
    }

    /// Вариант с явной стоимостью хеширования (используется в тестах).
    pub fn from_password_with_cost(password: &str, cost: u32) -> Result<Self, PasswordHashError> {
        if password.chars().count() < Self::MIN_PASSWORD_LENGTH {
            return Err(PasswordHashError::PasswordTooShort);
        }
        let hash = bcrypt::hash(password, cost)
            .map_err(|error| PasswordHashError::HashingFailed(error.to_string()))?;
        Ok(Self(hash))
    }

    /// Восстанавливает объект-значение из хеша, прочитанного из базы данных.
    pub fn from_hash(value: String) -> Result<Self, PasswordHashError> {
        let trimmed = value.trim();
        if trimmed.len() < 20 || !trimmed.starts_with("$2") {
            return Err(PasswordHashError::InvalidHash);
        }
        Ok(Self(trimmed.to_string()))
    }

    pub fn value(&self) -> &str {
        &self.0
    }

    pub fn verify(&self, password: &str) -> bool {
        bcrypt::verify(password, &self.0).unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn password_is_verified_against_hash() {
        let hash = PasswordHash::from_password_with_cost("admin123", 4).unwrap();
        assert!(hash.verify("admin123"));
        assert!(!hash.verify("admin124"));
        assert!(!hash.value().contains("admin123"));
    }

    #[test]
    fn short_password_is_rejected() {
        assert!(matches!(
            PasswordHash::from_password_with_cost("123", 4),
            Err(PasswordHashError::PasswordTooShort)
        ));
    }

    #[test]
    fn plain_text_is_not_a_valid_hash() {
        assert!(matches!(
            PasswordHash::from_hash("admin123".to_string()),
            Err(PasswordHashError::InvalidHash)
        ));
    }
}