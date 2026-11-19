use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct EntityId(Uuid);

#[derive(Debug, thiserror::Error)]
pub enum EntityIdError {
    #[error("entity id is empty")]
    EmptyId,
    #[error("entity id is invalid")]
    InvalidId,
}

impl EntityId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn create(id: Uuid) -> Result<Self, EntityIdError> {
        if id.is_nil() {
            return Err(EntityIdError::EmptyId);
        }
        if id.get_variant() != uuid::Variant::RFC4122
            || id.get_version() != Some(uuid::Version::Random)
        {
            return Err(EntityIdError::InvalidId);
        }
        Ok(Self(id))
    }

    pub fn value(&self) -> Uuid {
        self.0
    }
}
