use crate::entities::value_objects::{
    Address, AddressError, EntityId, EntityIdError, Name, NameError, RecordLifetime,
    RecordLifetimeError,
};

#[doc = "Издательство"]
#[derive(Debug, Clone)]
pub struct Publisher {
    pub(crate) id: EntityId,
    pub(crate) name: Name,
    pub(crate) address: Address,
    pub(crate) record_lifetime: RecordLifetime,
}

#[derive(Debug, thiserror::Error)]
pub enum PublisherError {
    #[error("publisher id error: {0}")]
    Id(#[from] EntityIdError),
    #[error("publisher name error: {0}")]
    Name(#[from] NameError),
    #[error("publisher address error: {0}")]
    Address(#[from] AddressError),
    #[error("publisher record lifetime error: {0}")]
    RecordLifetime(#[from] RecordLifetimeError),
}

impl Publisher {
    pub fn create_new(name: Name, address: Address) -> Result<Self, PublisherError> {
        let record_lifetime = RecordLifetime::create_new()?;
        Ok(Self::create(
            EntityId::new(),
            name,
            address,
            record_lifetime,
        ))
    }

    pub fn create(
        id: EntityId,
        name: Name,
        address: Address,
        record_lifetime: RecordLifetime,
    ) -> Self {
        Self {
            id,
            name,
            address,
            record_lifetime,
        }
    }
}
