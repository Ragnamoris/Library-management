/// Роль работника библиотеки. Определяет доступный набор сценариев использования.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StaffRole {
    Admin,
    Librarian,
}

#[derive(Debug, thiserror::Error)]
pub enum StaffRoleError {
    #[error("неизвестная роль работника: {0}")]
    Unknown(String),
}

impl StaffRole {
    pub fn parse(value: &str) -> Result<Self, StaffRoleError> {
        match value.trim().to_ascii_lowercase().as_str() {
            "admin" => Ok(StaffRole::Admin),
            "librarian" => Ok(StaffRole::Librarian),
            other => Err(StaffRoleError::Unknown(other.to_string())),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            StaffRole::Admin => "admin",
            StaffRole::Librarian => "librarian",
        }
    }

    pub fn title(&self) -> &'static str {
        match self {
            StaffRole::Admin => "Администратор",
            StaffRole::Librarian => "Библиотекарь",
        }
    }

    /// Управление справочниками издательств и книг.
    pub fn can_manage_catalog(&self) -> bool {
        matches!(self, StaffRole::Admin)
    }

    /// Регистрация и редактирование читателей.
    pub fn can_manage_readers(&self) -> bool {
        matches!(self, StaffRole::Admin)
    }

    /// Оформление выдач и приём возвратов.
    pub fn can_issue_loans(&self) -> bool {
        true
    }

    /// Формирование отчётов.
    pub fn can_generate_reports(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn role_round_trip() {
        for role in [StaffRole::Admin, StaffRole::Librarian] {
            assert_eq!(StaffRole::parse(role.as_str()).unwrap(), role);
        }
    }

    #[test]
    fn unknown_role_is_rejected() {
        assert!(StaffRole::parse("superuser").is_err());
    }

    #[test]
    fn librarian_cannot_manage_catalog() {
        assert!(!StaffRole::Librarian.can_manage_catalog());
        assert!(!StaffRole::Librarian.can_manage_readers());
        assert!(StaffRole::Librarian.can_issue_loans());
    }

    #[test]
    fn admin_manages_catalog() {
        assert!(StaffRole::Admin.can_manage_catalog());
        assert!(StaffRole::Admin.can_manage_readers());
    }
}