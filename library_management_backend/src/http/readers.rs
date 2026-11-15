use ohkami::claw::Path;
use ohkami::{fang::Context, Json, Response};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::contracts::specifications::ReaderSpecification;
use crate::contracts::ReadersStorage;
use crate::entities::value_objects::{CardNumber, Name, PhoneNumber};
use crate::entities::Reader;
use crate::http::context::RequestContext;
use crate::http::envelope::Envelope;

#[derive(Deserialize)]
pub struct AddReaderPayload {
    pub full_name: String,
    pub phone_number: String,
    pub card_number: String,
}

#[derive(Deserialize)]
pub struct UpdateReaderPayload {
    pub full_name: String,
    pub phone_number: String,
    pub card_number: String,
}

#[derive(Serialize)]
pub struct ReaderResponse {
    pub id: String,
    pub full_name: String,
    pub phone_number: String,
    pub card_number: String,
}

fn reader_response(reader: &Reader) -> ReaderResponse {
    ReaderResponse {
        id: reader.id.value().to_string(),
        full_name: reader.full_name.value().to_string(),
        phone_number: reader.phone_number.value().to_string(),
        card_number: reader.card_number.value().to_string(),
    }
}

pub async fn add_reader(
    Json(payload): Json<AddReaderPayload>,
    Context(context): Context<'_, RequestContext>,
) -> Response {
    let full_name = match Name::new(payload.full_name) {
        Ok(value) => value,
        Err(error) => return Envelope::<()>::bad_request(Some(error.to_string())),
    };
    let phone_number = match PhoneNumber::new(payload.phone_number) {
        Ok(value) => value,
        Err(error) => return Envelope::<()>::bad_request(Some(error.to_string())),
    };
    let card_number = match CardNumber::new(payload.card_number) {
        Ok(value) => value,
        Err(error) => return Envelope::<()>::bad_request(Some(error.to_string())),
    };

    let by_phone = match context
        .readers
        .get(ReaderSpecification::new().with_phone_number(phone_number.clone()))
        .await
    {
        Ok(value) => value,
        Err(error) => return Envelope::<()>::internal_error(Some(error.to_string())),
    };
    if by_phone.is_some() {
        return Envelope::<()>::conflict(Some(
            "reader with this phone number already exists".to_string(),
        ));
    }

    let by_card = match context
        .readers
        .get(ReaderSpecification::new().with_card_number(card_number.clone()))
        .await
    {
        Ok(value) => value,
        Err(error) => return Envelope::<()>::internal_error(Some(error.to_string())),
    };
    if by_card.is_some() {
        return Envelope::<()>::conflict(Some(
            "reader with this card number already exists".to_string(),
        ));
    }

    let reader = match Reader::create_new(full_name, phone_number, card_number) {
        Ok(value) => value,
        Err(error) => return Envelope::<()>::bad_request(Some(error.to_string())),
    };
    let body = reader_response(&reader);

    match context.readers.add(reader).await {
        Ok(()) => Envelope::created(Some(body)),
        Err(error) => Envelope::<()>::internal_error(Some(error.to_string())),
    }
}

pub async fn get_reader(
    Path(id): Path<&str>,
    Context(context): Context<'_, RequestContext>,
) -> Response {
    let Ok(id) = Uuid::parse_str(id) else {
        return Envelope::<()>::bad_request(Some("invalid reader id".to_string()));
    };

    match context
        .readers
        .get(ReaderSpecification::new().with_id(id))
        .await
    {
        Ok(Some(reader)) => Envelope::ok(Some(reader_response(&reader))),
        Ok(None) => Envelope::<()>::not_found(Some("reader not found".to_string())),
        Err(error) => Envelope::<()>::internal_error(Some(error.to_string())),
    }
}

pub async fn update_reader(
    Path(id): Path<&str>,
    Json(payload): Json<UpdateReaderPayload>,
    Context(context): Context<'_, RequestContext>,
) -> Response {
    let Ok(id) = Uuid::parse_str(id) else {
        return Envelope::<()>::bad_request(Some("invalid reader id".to_string()));
    };
    let full_name = match Name::new(payload.full_name) {
        Ok(value) => value,
        Err(error) => return Envelope::<()>::bad_request(Some(error.to_string())),
    };
    let phone_number = match PhoneNumber::new(payload.phone_number) {
        Ok(value) => value,
        Err(error) => return Envelope::<()>::bad_request(Some(error.to_string())),
    };
    let card_number = match CardNumber::new(payload.card_number) {
        Ok(value) => value,
        Err(error) => return Envelope::<()>::bad_request(Some(error.to_string())),
    };

    let mut reader = match context
        .readers
        .get(ReaderSpecification::new().with_id(id))
        .await
    {
        Ok(Some(value)) => value,
        Ok(None) => return Envelope::<()>::not_found(Some("reader not found".to_string())),
        Err(error) => return Envelope::<()>::internal_error(Some(error.to_string())),
    };

    let phone_owner = match context
        .readers
        .get(ReaderSpecification::new().with_phone_number(phone_number.clone()))
        .await
    {
        Ok(value) => value,
        Err(error) => return Envelope::<()>::internal_error(Some(error.to_string())),
    };
    if let Some(other) = phone_owner {
        if other.id.value() != reader.id.value() {
            return Envelope::<()>::conflict(Some(
                "phone number is already used by another reader".to_string(),
            ));
        }
    }

    reader.full_name = full_name;
    reader.phone_number = phone_number;
    reader.card_number = card_number;

    match context.readers.update(&reader).await {
        Ok(()) => Envelope::ok(Some(reader_response(&reader))),
        Err(error) => Envelope::<()>::internal_error(Some(error.to_string())),
    }
}

pub async fn remove_reader(
    Path(id): Path<&str>,
    Context(context): Context<'_, RequestContext>,
) -> Response {
    let Ok(id) = Uuid::parse_str(id) else {
        return Envelope::<()>::bad_request(Some("invalid reader id".to_string()));
    };

    let reader = match context
        .readers
        .get(ReaderSpecification::new().with_id(id))
        .await
    {
        Ok(Some(value)) => value,
        Ok(None) => return Envelope::<()>::not_found(Some("reader not found".to_string())),
        Err(error) => return Envelope::<()>::internal_error(Some(error.to_string())),
    };

    match context.readers.remove(reader).await {
        Ok(()) => Envelope::<()>::ok(None),
        Err(error) => Envelope::<()>::internal_error(Some(error.to_string())),
    }
}
