use uuid::Uuid;

use crate::entities::value_objects::{Address, CardNumber, InventoryNumber, Name, PhoneNumber};

#[doc = "Спецификация издательства: определяет, какое издательство нужно достать из базы данных."]
#[derive(Debug, Clone, Default)]
pub struct PublisherSpecification {
    pub id: Option<Uuid>,
    pub name: Option<Name>,
    pub address: Option<Address>,
}

impl PublisherSpecification {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_id(mut self, id: Uuid) -> Self {
        self.id = Some(id);
        self
    }

    pub fn with_name(mut self, name: Name) -> Self {
        self.name = Some(name);
        self
    }
}

#[doc = "Спецификация читателя: определяет, какого читателя нужно достать из базы данных."]
#[derive(Debug, Clone, Default)]
pub struct ReaderSpecification {
    pub id: Option<Uuid>,
    pub full_name: Option<Name>,
    pub phone_number: Option<PhoneNumber>,
    pub card_number: Option<CardNumber>,
}

impl ReaderSpecification {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_id(mut self, id: Uuid) -> Self {
        self.id = Some(id);
        self
    }

    pub fn with_phone_number(mut self, phone_number: PhoneNumber) -> Self {
        self.phone_number = Some(phone_number);
        self
    }

    pub fn with_card_number(mut self, card_number: CardNumber) -> Self {
        self.card_number = Some(card_number);
        self
    }
}

#[doc = "Спецификация книги: определяет, какую книгу нужно достать из базы данных."]
#[derive(Debug, Clone, Default)]
pub struct BookSpecification {
    pub id: Option<Uuid>,
    pub publisher_id: Option<Uuid>,
    pub title: Option<Name>,
    pub author: Option<Name>,
    pub inventory_number: Option<InventoryNumber>,
}

impl BookSpecification {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_id(mut self, id: Uuid) -> Self {
        self.id = Some(id);
        self
    }

    pub fn with_inventory_number(mut self, inventory_number: InventoryNumber) -> Self {
        self.inventory_number = Some(inventory_number);
        self
    }
}

#[doc = "Спецификация выдачи: определяет, какую выдачу нужно достать из базы данных."]
#[derive(Debug, Clone, Default)]
pub struct LoanSpecification {
    pub id: Option<Uuid>,
    pub book_id: Option<Uuid>,
    pub reader_id: Option<Uuid>,
    pub only_active: bool,
}

impl LoanSpecification {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_id(mut self, id: Uuid) -> Self {
        self.id = Some(id);
        self
    }

    pub fn with_book_id(mut self, book_id: Uuid) -> Self {
        self.book_id = Some(book_id);
        self
    }

    pub fn with_only_active(mut self) -> Self {
        self.only_active = true;
        self
    }
}
