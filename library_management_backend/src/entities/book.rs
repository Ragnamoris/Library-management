use crate::entities::value_objects::{
    EntityId, InventoryNumber, Name, PublicationYear, RecordLifetime, RecordLifetimeError,
};
use crate::entities::Publisher;

#[doc = "Книга"]
#[derive(Debug, Clone)]
pub struct Book {
    pub(crate) id: EntityId,
    pub(crate) publisher_id: EntityId,
    pub(crate) title: Name,
    pub(crate) author: Name,
    pub(crate) publication_year: PublicationYear,
    pub(crate) inventory_number: InventoryNumber,
    pub(crate) record_lifetime: RecordLifetime,
}

#[derive(Debug, thiserror::Error)]
pub enum BookError {
    #[error("book record lifetime error: {0}")]
    RecordLifetime(#[from] RecordLifetimeError),
}

impl Book {
    pub fn create_new(
        publisher: &Publisher,
        title: Name,
        author: Name,
        publication_year: PublicationYear,
        inventory_number: InventoryNumber,
    ) -> Result<Self, BookError> {
        let record_lifetime = RecordLifetime::create_new()?;
        Ok(Self::create(
            EntityId::new(),
            publisher.id,
            title,
            author,
            publication_year,
            inventory_number,
            record_lifetime,
        ))
    }

    pub fn create(
        id: EntityId,
        publisher_id: EntityId,
        title: Name,
        author: Name,
        publication_year: PublicationYear,
        inventory_number: InventoryNumber,
        record_lifetime: RecordLifetime,
    ) -> Self {
        Self {
            id,
            publisher_id,
            title,
            author,
            publication_year,
            inventory_number,
            record_lifetime,
        }
    }
}
