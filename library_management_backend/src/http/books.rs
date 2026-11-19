use ohkami::claw::Path;
use ohkami::{fang::Context, Json, Response};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::contracts::specifications::{BookSpecification, PublisherSpecification};
use crate::contracts::{BooksStorage, PublishersStorage};
use crate::entities::value_objects::{InventoryNumber, Name, PublicationYear};
use crate::entities::Book;
use crate::http::context::RequestContext;
use crate::http::envelope::Envelope;

#[derive(Deserialize)]
pub struct AddBookPayload {
    pub publisher_id: String,
    pub title: String,
    pub author: String,
    pub publication_year: i32,
    pub inventory_number: String,
}

#[derive(Deserialize)]
pub struct UpdateBookPayload {
    pub title: String,
    pub author: String,
    pub publication_year: i32,
    pub inventory_number: String,
}

#[derive(Serialize)]
pub struct BookResponse {
    pub id: String,
    pub publisher_id: String,
    pub title: String,
    pub author: String,
    pub publication_year: i32,
    pub inventory_number: String,
}

fn book_response(book: &Book) -> BookResponse {
    BookResponse {
        id: book.id.value().to_string(),
        publisher_id: book.publisher_id.value().to_string(),
        title: book.title.value().to_string(),
        author: book.author.value().to_string(),
        publication_year: book.publication_year.value(),
        inventory_number: book.inventory_number.value().to_string(),
    }
}

pub async fn add_book(
    Json(payload): Json<AddBookPayload>,
    Context(context): Context<'_, RequestContext>,
) -> Response {
    let Ok(publisher_id) = Uuid::parse_str(&payload.publisher_id) else {
        return Envelope::<()>::bad_request(Some("invalid publisher id".to_string()));
    };
    let title = match Name::new(payload.title) {
        Ok(value) => value,
        Err(error) => return Envelope::<()>::bad_request(Some(error.to_string())),
    };
    let author = match Name::new(payload.author) {
        Ok(value) => value,
        Err(error) => return Envelope::<()>::bad_request(Some(error.to_string())),
    };
    let publication_year = match PublicationYear::new(payload.publication_year) {
        Ok(value) => value,
        Err(error) => return Envelope::<()>::bad_request(Some(error.to_string())),
    };
    let inventory_number = match InventoryNumber::new(payload.inventory_number) {
        Ok(value) => value,
        Err(error) => return Envelope::<()>::bad_request(Some(error.to_string())),
    };

    let publisher = match context
        .publishers
        .get(PublisherSpecification::new().with_id(publisher_id))
        .await
    {
        Ok(Some(value)) => value,
        Ok(None) => return Envelope::<()>::not_found(Some("publisher not found".to_string())),
        Err(error) => return Envelope::<()>::internal_error(Some(error.to_string())),
    };

    let duplicate = match context
        .books
        .get(BookSpecification::new().with_inventory_number(inventory_number.clone()))
        .await
    {
        Ok(value) => value,
        Err(error) => return Envelope::<()>::internal_error(Some(error.to_string())),
    };
    if duplicate.is_some() {
        return Envelope::<()>::conflict(Some(
            "book with this inventory number already exists".to_string(),
        ));
    }

    let book = match Book::create_new(
        &publisher,
        title,
        author,
        publication_year,
        inventory_number,
    ) {
        Ok(value) => value,
        Err(error) => return Envelope::<()>::bad_request(Some(error.to_string())),
    };
    let body = book_response(&book);

    match context.books.add(book).await {
        Ok(()) => Envelope::created(Some(body)),
        Err(error) => Envelope::<()>::internal_error(Some(error.to_string())),
    }
}

pub async fn get_book(
    Path(id): Path<&str>,
    Context(context): Context<'_, RequestContext>,
) -> Response {
    let Ok(id) = Uuid::parse_str(id) else {
        return Envelope::<()>::bad_request(Some("invalid book id".to_string()));
    };

    match context
        .books
        .get(BookSpecification::new().with_id(id))
        .await
    {
        Ok(Some(book)) => Envelope::ok(Some(book_response(&book))),
        Ok(None) => Envelope::<()>::not_found(Some("book not found".to_string())),
        Err(error) => Envelope::<()>::internal_error(Some(error.to_string())),
    }
}

pub async fn update_book(
    Path(id): Path<&str>,
    Json(payload): Json<UpdateBookPayload>,
    Context(context): Context<'_, RequestContext>,
) -> Response {
    let Ok(id) = Uuid::parse_str(id) else {
        return Envelope::<()>::bad_request(Some("invalid book id".to_string()));
    };
    let title = match Name::new(payload.title) {
        Ok(value) => value,
        Err(error) => return Envelope::<()>::bad_request(Some(error.to_string())),
    };
    let author = match Name::new(payload.author) {
        Ok(value) => value,
        Err(error) => return Envelope::<()>::bad_request(Some(error.to_string())),
    };
    let publication_year = match PublicationYear::new(payload.publication_year) {
        Ok(value) => value,
        Err(error) => return Envelope::<()>::bad_request(Some(error.to_string())),
    };
    let inventory_number = match InventoryNumber::new(payload.inventory_number) {
        Ok(value) => value,
        Err(error) => return Envelope::<()>::bad_request(Some(error.to_string())),
    };

    let mut book = match context
        .books
        .get(BookSpecification::new().with_id(id))
        .await
    {
        Ok(Some(value)) => value,
        Ok(None) => return Envelope::<()>::not_found(Some("book not found".to_string())),
        Err(error) => return Envelope::<()>::internal_error(Some(error.to_string())),
    };

    let duplicate = match context
        .books
        .get(BookSpecification::new().with_inventory_number(inventory_number.clone()))
        .await
    {
        Ok(value) => value,
        Err(error) => return Envelope::<()>::internal_error(Some(error.to_string())),
    };
    if let Some(other) = duplicate {
        if other.id.value() != book.id.value() {
            return Envelope::<()>::conflict(Some(
                "inventory number is already used by another book".to_string(),
            ));
        }
    }

    book.title = title;
    book.author = author;
    book.publication_year = publication_year;
    book.inventory_number = inventory_number;

    match context.books.update(&book).await {
        Ok(()) => Envelope::ok(Some(book_response(&book))),
        Err(error) => Envelope::<()>::internal_error(Some(error.to_string())),
    }
}

pub async fn remove_book(
    Path(id): Path<&str>,
    Context(context): Context<'_, RequestContext>,
) -> Response {
    let Ok(id) = Uuid::parse_str(id) else {
        return Envelope::<()>::bad_request(Some("invalid book id".to_string()));
    };

    let book = match context
        .books
        .get(BookSpecification::new().with_id(id))
        .await
    {
        Ok(Some(value)) => value,
        Ok(None) => return Envelope::<()>::not_found(Some("book not found".to_string())),
        Err(error) => return Envelope::<()>::internal_error(Some(error.to_string())),
    };

    match context.books.remove(book).await {
        Ok(()) => Envelope::<()>::ok(None),
        Err(error) => Envelope::<()>::internal_error(Some(error.to_string())),
    }
}
