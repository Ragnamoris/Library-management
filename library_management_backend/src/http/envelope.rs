use ohkami::{Response, Status};
use serde::Serialize;
use time::UtcDateTime;

fn serialize_utc_datetime<S>(value: &UtcDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let text = value
        .format(&time::format_description::well_known::Rfc3339)
        .map_err(serde::ser::Error::custom)?;
    serializer.serialize_str(&text)
}

#[derive(Serialize)]
pub struct Envelope<T>
where
    T: Serialize,
{
    pub data: Option<T>,
    pub error: Option<String>,
    pub status: i32,
    #[serde(serialize_with = "serialize_utc_datetime")]
    pub date: UtcDateTime,
}

impl<T> Envelope<T>
where
    T: Serialize,
{
    fn resolve_ohkami_status_code(&self) -> Status {
        match self.status {
            100..=103
            | 200..=208
            | 226
            | 300..=304
            | 307..=308
            | 400..=401
            | 403..=418
            | 421..=424
            | 426
            | 428..=429
            | 431
            | 451
            | 500..=508
            | 510..=511 => Status::from(self.status as u16),
            _ => Status::InternalServerError,
        }
    }

    pub fn into_response(self) -> Response {
        let status = self.resolve_ohkami_status_code();
        Response::new(status).with_json(self)
    }

    fn current_error(error: Option<String>) -> Self {
        Envelope {
            data: None,
            error,
            status: 400,
            date: UtcDateTime::now(),
        }
    }

    fn current_success(data: Option<T>) -> Self {
        Envelope {
            data,
            error: None,
            status: 200,
            date: UtcDateTime::now(),
        }
    }

    pub fn bad_request(error: Option<String>) -> Response {
        Self::current_error(error).into_response()
    }

    pub fn ok(data: Option<T>) -> Response {
        Self::current_success(data).into_response()
    }

    pub fn created(data: Option<T>) -> Response {
        let mut envelope = Self::current_success(data);
        envelope.status = 201;
        envelope.into_response()
    }

    pub fn not_found(error: Option<String>) -> Response {
        let mut envelope = Self::current_error(error);
        envelope.status = 404;
        envelope.into_response()
    }

    pub fn conflict(error: Option<String>) -> Response {
        let mut envelope = Self::current_error(error);
        envelope.status = 409;
        envelope.into_response()
    }

    pub fn internal_error(error: Option<String>) -> Response {
        let mut envelope = Self::current_error(error);
        envelope.status = 500;
        envelope.into_response()
    }
}
