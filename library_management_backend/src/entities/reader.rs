use crate::entities::value_objects::{
    CardNumber, CardNumberError, EntityId, EntityIdError, Name, NameError, PhoneNumber,
    PhoneNumberError, RecordLifetime, RecordLifetimeError,
};

#[doc = "Читатель"]
#[derive(Debug, Clone)]
pub struct Reader {
    pub(crate) id: EntityId,
    pub(crate) full_name: Name,
    pub(crate) phone_number: PhoneNumber,
    pub(crate) card_number: CardNumber,
    pub(crate) record_lifetime: RecordLifetime,
}

#[derive(Debug, thiserror::Error)]
pub enum ReaderError {
    #[error("reader id error: {0}")]
    Id(#[from] EntityIdError),
    #[error("reader name error: {0}")]
    Name(#[from] NameError),
    #[error("reader phone number error: {0}")]
    PhoneNumber(#[from] PhoneNumberError),
    #[error("reader card number error: {0}")]
    CardNumber(#[from] CardNumberError),
    #[error("reader record lifetime error: {0}")]
    RecordLifetime(#[from] RecordLifetimeError),
}

impl Reader {
    pub fn create_new(
        full_name: Name,
        phone_number: PhoneNumber,
        card_number: CardNumber,
    ) -> Result<Self, ReaderError> {
        let record_lifetime = RecordLifetime::create_new()?;
        Ok(Self::create(
            EntityId::new(),
            full_name,
            phone_number,
            card_number,
            record_lifetime,
        ))
    }

    pub fn create(
        id: EntityId,
        full_name: Name,
        phone_number: PhoneNumber,
        card_number: CardNumber,
        record_lifetime: RecordLifetime,
    ) -> Self {
        Self {
            id,
            full_name,
            phone_number,
            card_number,
            record_lifetime,
        }
    }
}
