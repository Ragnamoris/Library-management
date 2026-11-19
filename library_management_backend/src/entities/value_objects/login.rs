/// Логин работника библиотеки.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Login(String);

#[derive(Debug, thiserror::Error)]
pub enum LoginError {
    #[error("логин должен содержать от 3 до 32 символов")]
    InvalidLength,
    #[error("логин может содержать только латинские буквы, цифры и знаки '.', '_', '-'")]
    InvalidCharacters,
}

impl Login {
    pub const MIN_LENGTH: usize = 3;
    pub const MAX_LENGTH: usize = 32;

    pub fn new(value: String) -> Result<Self, LoginError> {
        let normalized = value.trim().to_ascii_lowercase();

        if normalized.len() < Self::MIN_LENGTH || normalized.len() > Self::MAX_LENGTH {
            return Err(LoginError::InvalidLength);
        }

        let valid = normalized
            .chars()
            .all(|symbol| symbol.is_ascii_alphanumeric() || matches!(symbol, '.' | '_' | '-'));
        if !valid {
            return Err(LoginError::InvalidCharacters);
        }

        Ok(Self(normalized))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn login_is_normalized_to_lowercase() {
        let login = Login::new("  Admin  ".to_string()).unwrap();
        assert_eq!(login.value(), "admin");
    }

    #[test]
    fn too_short_login_is_rejected() {
        assert!(matches!(
            Login::new("ab".to_string()),
            Err(LoginError::InvalidLength)
        ));
    }

    #[test]
    fn cyrillic_login_is_rejected() {
        assert!(matches!(
            Login::new("библиотекарь".to_string()),
            Err(LoginError::InvalidCharacters)
        ));
    }

    #[test]
    fn allowed_symbols_are_accepted() {
        assert!(Login::new("head.librarian_1".to_string()).is_ok());
    }
}