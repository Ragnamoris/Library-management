use crate::entities::value_objects::{
    EntityId, Login, Name, PasswordHash, RecordLifetime, RecordLifetimeError, StaffRole,
};

#[doc = "Работник библиотеки — пользователь системы"]
#[derive(Debug, Clone)]
pub struct Staff {
    pub(crate) id: EntityId,
    pub(crate) login: Login,
    pub(crate) password_hash: PasswordHash,
    pub(crate) full_name: Name,
    pub(crate) role: StaffRole,
    pub(crate) record_lifetime: RecordLifetime,
}

#[derive(Debug, thiserror::Error)]
pub enum StaffError {
    #[error("staff record lifetime error: {0}")]
    RecordLifetime(#[from] RecordLifetimeError),
}

impl Staff {
    pub fn create_new(
        login: Login,
        password_hash: PasswordHash,
        full_name: Name,
        role: StaffRole,
    ) -> Result<Self, StaffError> {
        let record_lifetime = RecordLifetime::create_new()?;
        Ok(Self::create(
            EntityId::new(),
            login,
            password_hash,
            full_name,
            role,
            record_lifetime,
        ))
    }

    pub fn create(
        id: EntityId,
        login: Login,
        password_hash: PasswordHash,
        full_name: Name,
        role: StaffRole,
        record_lifetime: RecordLifetime,
    ) -> Self {
        Self {
            id,
            login,
            password_hash,
            full_name,
            role,
            record_lifetime,
        }
    }

    pub fn id(&self) -> EntityId {
        self.id
    }

    pub fn login(&self) -> &Login {
        &self.login
    }

    pub fn password_hash(&self) -> &PasswordHash {
        &self.password_hash
    }

    pub fn full_name(&self) -> &Name {
        &self.full_name
    }

    pub fn role(&self) -> StaffRole {
        self.role
    }

    pub fn record_lifetime(&self) -> &RecordLifetime {
        &self.record_lifetime
    }

    pub fn is_admin(&self) -> bool {
        matches!(self.role, StaffRole::Admin)
    }

    /// Проверка пароля при входе в систему.
    pub fn verify_password(&self, password: &str) -> bool {
        self.password_hash.verify(password)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_staff() -> Staff {
        Staff::create_new(
            Login::new("librarian".to_string()).unwrap(),
            PasswordHash::from_password_with_cost("lib123", 4).unwrap(),
            Name::new("Иванов Иван Иванович".to_string()).unwrap(),
            StaffRole::Librarian,
        )
        .unwrap()
    }

    #[test]
    fn staff_verifies_correct_password() {
        let staff = sample_staff();
        assert!(staff.verify_password("lib123"));
        assert!(!staff.verify_password("wrong-password"));
    }

    #[test]
    fn staff_role_defines_permissions() {
        let staff = sample_staff();
        assert!(!staff.is_admin());
        assert!(!staff.role().can_manage_catalog());
        assert!(staff.role().can_issue_loans());
    }

    #[test]
    fn staff_has_identifier_and_lifetime() {
        let staff = sample_staff();
        assert!(!staff.id().value().is_nil());
        assert!(staff.record_lifetime().deleted_at().is_none());
    }
}