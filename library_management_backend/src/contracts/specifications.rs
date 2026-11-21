use time::UtcDateTime;
use uuid::Uuid;

use crate::entities::value_objects::{
    Address, CardNumber, InventoryNumber, Login, Name, PhoneNumber, StaffRole,
};

/// Спецификация издательства: описывает, какие записи нужно достать из базы данных.
#[derive(Debug, Clone, Default)]
pub struct PublisherSpecification {
    pub id: Option<Uuid>,
    pub name: Option<Name>,
    pub address: Option<Address>,
    pub search: Option<String>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
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
    pub fn with_search(mut self, search: impl Into<String>) -> Self {
        self.search = Some(search.into());
        self
    }
    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn with_offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }
}

/// Спецификация читателя.
#[derive(Debug, Clone, Default)]
pub struct ReaderSpecification {
    pub id: Option<Uuid>,
    pub full_name: Option<Name>,
    pub phone_number: Option<PhoneNumber>,
    pub card_number: Option<CardNumber>,
    pub search: Option<String>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
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
    pub fn with_search(mut self, search: impl Into<String>) -> Self {
        self.search = Some(search.into());
        self
    }
    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn with_offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }
}

/// Спецификация книги.
#[derive(Debug, Clone, Default)]
pub struct BookSpecification {
    pub id: Option<Uuid>,
    pub publisher_id: Option<Uuid>,
    pub title: Option<Name>,
    pub author: Option<Name>,
    pub inventory_number: Option<InventoryNumber>,
    pub search: Option<String>,
    pub only_available: bool,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

impl BookSpecification {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_id(mut self, id: Uuid) -> Self {
        self.id = Some(id);
        self
    }
    pub fn with_publisher_id(mut self, publisher_id: Uuid) -> Self {
        self.publisher_id = Some(publisher_id);
        self
    }
    pub fn with_inventory_number(mut self, inventory_number: InventoryNumber) -> Self {
        self.inventory_number = Some(inventory_number);
        self
    }
    pub fn with_search(mut self, search: impl Into<String>) -> Self {
        self.search = Some(search.into());
        self
    }
    pub fn with_only_available(mut self) -> Self {
        self.only_available = true;
        self
    }
    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn with_offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }
}

/// Спецификация выдачи книги.
#[derive(Debug, Clone, Default)]
pub struct LoanSpecification {
    pub id: Option<Uuid>,
    pub book_id: Option<Uuid>,
    pub reader_id: Option<Uuid>,
    pub only_active: bool,
    pub only_overdue: bool,
    pub issued_from: Option<UtcDateTime>,
    pub issued_to: Option<UtcDateTime>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
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
    pub fn with_reader_id(mut self, reader_id: Uuid) -> Self {
        self.reader_id = Some(reader_id);
        self
    }
    pub fn with_only_active(mut self) -> Self {
        self.only_active = true;
        self
    }
    pub fn with_only_overdue(mut self) -> Self {
        self.only_overdue = true;
        self
    }
    pub fn with_period(mut self, from: UtcDateTime, to: UtcDateTime) -> Self {
        self.issued_from = Some(from);
        self.issued_to = Some(to);
        self
    }
    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn with_offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }
}

/// Спецификация работника библиотеки.
#[derive(Debug, Clone, Default)]
pub struct StaffSpecification {
    pub id: Option<Uuid>,
    pub login: Option<Login>,
    pub role: Option<StaffRole>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

impl StaffSpecification {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_id(mut self, id: Uuid) -> Self {
        self.id = Some(id);
        self
    }
    pub fn with_login(mut self, login: Login) -> Self {
        self.login = Some(login);
        self
    }
    pub fn with_role(mut self, role: StaffRole) -> Self {
        self.role = Some(role);
        self
    }
}