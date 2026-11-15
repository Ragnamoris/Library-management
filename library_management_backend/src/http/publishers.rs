use ohkami::claw::Path;
use ohkami::{fang::Context, Json, Response};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::contracts::specifications::PublisherSpecification;
use crate::contracts::PublishersStorage;
use crate::entities::value_objects::{Address, Name};
use crate::entities::Publisher;
use crate::http::context::RequestContext;
use crate::http::envelope::Envelope;

#[derive(Deserialize)]
pub struct AddPublisherPayload {
    pub name: String,
    pub address: String,
}

#[derive(Deserialize)]
pub struct UpdatePublisherPayload {
    pub name: String,
    pub address: String,
}

#[derive(Serialize)]
pub struct PublisherResponse {
    pub id: String,
    pub name: String,
    pub address: String,
}

fn publisher_response(publisher: &Publisher) -> PublisherResponse {
    PublisherResponse {
        id: publisher.id.value().to_string(),
        name: publisher.name.value().to_string(),
        address: publisher.address.value().to_string(),
    }
}

pub async fn add_publisher(
    Json(payload): Json<AddPublisherPayload>,
    Context(context): Context<'_, RequestContext>,
) -> Response {
    let name = match Name::new(payload.name) {
        Ok(value) => value,
        Err(error) => return Envelope::<()>::bad_request(Some(error.to_string())),
    };
    let address = match Address::new(payload.address) {
        Ok(value) => value,
        Err(error) => return Envelope::<()>::bad_request(Some(error.to_string())),
    };

    let existing = match context
        .publishers
        .get(PublisherSpecification::new().with_name(name.clone()))
        .await
    {
        Ok(value) => value,
        Err(error) => return Envelope::<()>::internal_error(Some(error.to_string())),
    };
    if existing.is_some() {
        return Envelope::<()>::conflict(Some(
            "publisher with this name already exists".to_string(),
        ));
    }

    let publisher = match Publisher::create_new(name, address) {
        Ok(value) => value,
        Err(error) => return Envelope::<()>::bad_request(Some(error.to_string())),
    };
    let body = publisher_response(&publisher);

    match context.publishers.add(publisher).await {
        Ok(()) => Envelope::created(Some(body)),
        Err(error) => Envelope::<()>::internal_error(Some(error.to_string())),
    }
}

pub async fn get_publisher(
    Path(id): Path<&str>,
    Context(context): Context<'_, RequestContext>,
) -> Response {
    let Ok(id) = Uuid::parse_str(id) else {
        return Envelope::<()>::bad_request(Some("invalid publisher id".to_string()));
    };

    match context
        .publishers
        .get(PublisherSpecification::new().with_id(id))
        .await
    {
        Ok(Some(publisher)) => Envelope::ok(Some(publisher_response(&publisher))),
        Ok(None) => Envelope::<()>::not_found(Some("publisher not found".to_string())),
        Err(error) => Envelope::<()>::internal_error(Some(error.to_string())),
    }
}

pub async fn update_publisher(
    Path(id): Path<&str>,
    Json(payload): Json<UpdatePublisherPayload>,
    Context(context): Context<'_, RequestContext>,
) -> Response {
    let Ok(id) = Uuid::parse_str(id) else {
        return Envelope::<()>::bad_request(Some("invalid publisher id".to_string()));
    };
    let name = match Name::new(payload.name) {
        Ok(value) => value,
        Err(error) => return Envelope::<()>::bad_request(Some(error.to_string())),
    };
    let address = match Address::new(payload.address) {
        Ok(value) => value,
        Err(error) => return Envelope::<()>::bad_request(Some(error.to_string())),
    };

    let mut publisher = match context
        .publishers
        .get(PublisherSpecification::new().with_id(id))
        .await
    {
        Ok(Some(value)) => value,
        Ok(None) => return Envelope::<()>::not_found(Some("publisher not found".to_string())),
        Err(error) => return Envelope::<()>::internal_error(Some(error.to_string())),
    };

    publisher.name = name;
    publisher.address = address;

    match context.publishers.update(&publisher).await {
        Ok(()) => Envelope::ok(Some(publisher_response(&publisher))),
        Err(error) => Envelope::<()>::internal_error(Some(error.to_string())),
    }
}

pub async fn remove_publisher(
    Path(id): Path<&str>,
    Context(context): Context<'_, RequestContext>,
) -> Response {
    let Ok(id) = Uuid::parse_str(id) else {
        return Envelope::<()>::bad_request(Some("invalid publisher id".to_string()));
    };

    let publisher = match context
        .publishers
        .get(PublisherSpecification::new().with_id(id))
        .await
    {
        Ok(Some(value)) => value,
        Ok(None) => return Envelope::<()>::not_found(Some("publisher not found".to_string())),
        Err(error) => return Envelope::<()>::internal_error(Some(error.to_string())),
    };

    match context.publishers.remove(publisher).await {
        Ok(()) => Envelope::<()>::ok(None),
        Err(error) => Envelope::<()>::internal_error(Some(error.to_string())),
    }
}
