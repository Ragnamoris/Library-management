use ohkami::claw::Path;
use ohkami::{fang::Context, Json, Response};
use serde::{Deserialize, Serialize};
use time::UtcDateTime;
use uuid::Uuid;

use crate::contracts::specifications::{BookSpecification, LoanSpecification, ReaderSpecification};
use crate::contracts::{BooksStorage, LoansStorage, ReadersStorage};
use crate::entities::loan::MAX_ACTIVE_LOANS;
use crate::entities::Loan;
use crate::http::context::RequestContext;
use crate::http::envelope::Envelope;

#[derive(Deserialize)]
pub struct AddLoanPayload {
    pub book_id: String,
    pub reader_id: String,
}

#[derive(Deserialize)]
pub struct UpdateLoanPayload {
    pub returned: bool,
}

#[derive(Serialize)]
pub struct LoanResponse {
    pub id: String,
    pub book_id: String,
    pub reader_id: String,
    pub issued_at: String,
    pub due_at: String,
    pub returned_at: Option<String>,
    pub is_overdue: bool,
}

fn loan_response(loan: &Loan) -> LoanResponse {
    LoanResponse {
        id: loan.id.value().to_string(),
        book_id: loan.book_id.value().to_string(),
        reader_id: loan.reader_id.value().to_string(),
        issued_at: loan.issued_at.to_string(),
        due_at: loan.due_at.to_string(),
        returned_at: loan.returned_at.map(|value| value.to_string()),
        is_overdue: loan.is_overdue(UtcDateTime::now()),
    }
}

pub async fn add_loan(
    Json(payload): Json<AddLoanPayload>,
    Context(context): Context<'_, RequestContext>,
) -> Response {
    let Ok(book_id) = Uuid::parse_str(&payload.book_id) else {
        return Envelope::<()>::bad_request(Some("invalid book id".to_string()));
    };
    let Ok(reader_id) = Uuid::parse_str(&payload.reader_id) else {
        return Envelope::<()>::bad_request(Some("invalid reader id".to_string()));
    };

    let book = match context
        .books
        .get(BookSpecification::new().with_id(book_id))
        .await
    {
        Ok(Some(value)) => value,
        Ok(None) => return Envelope::<()>::not_found(Some("book not found".to_string())),
        Err(error) => return Envelope::<()>::internal_error(Some(error.to_string())),
    };
    let reader = match context
        .readers
        .get(ReaderSpecification::new().with_id(reader_id))
        .await
    {
        Ok(Some(value)) => value,
        Ok(None) => return Envelope::<()>::not_found(Some("reader not found".to_string())),
        Err(error) => return Envelope::<()>::internal_error(Some(error.to_string())),
    };

    let active_loan = match context
        .loans
        .get(
            LoanSpecification::new()
                .with_book_id(book_id)
                .with_only_active(),
        )
        .await
    {
        Ok(value) => value,
        Err(error) => return Envelope::<()>::internal_error(Some(error.to_string())),
    };
    if active_loan.is_some() {
        return Envelope::<()>::conflict(Some(
            "book is already issued to another reader".to_string(),
        ));
    }

    let active_count = match context.loans.count_active_for_reader(reader_id).await {
        Ok(value) => value,
        Err(error) => return Envelope::<()>::internal_error(Some(error.to_string())),
    };
    if active_count >= MAX_ACTIVE_LOANS {
        return Envelope::<()>::conflict(Some(
            "reader already has the maximum number of books".to_string(),
        ));
    }

    let loan = match Loan::create_new(&book, &reader) {
        Ok(value) => value,
        Err(error) => return Envelope::<()>::bad_request(Some(error.to_string())),
    };
    let body = loan_response(&loan);

    match context.loans.add(loan).await {
        Ok(()) => Envelope::created(Some(body)),
        Err(error) => Envelope::<()>::internal_error(Some(error.to_string())),
    }
}

pub async fn get_loan(
    Path(id): Path<&str>,
    Context(context): Context<'_, RequestContext>,
) -> Response {
    let Ok(id) = Uuid::parse_str(id) else {
        return Envelope::<()>::bad_request(Some("invalid loan id".to_string()));
    };

    match context
        .loans
        .get(LoanSpecification::new().with_id(id))
        .await
    {
        Ok(Some(loan)) => Envelope::ok(Some(loan_response(&loan))),
        Ok(None) => Envelope::<()>::not_found(Some("loan not found".to_string())),
        Err(error) => Envelope::<()>::internal_error(Some(error.to_string())),
    }
}

pub async fn update_loan(
    Path(id): Path<&str>,
    Json(payload): Json<UpdateLoanPayload>,
    Context(context): Context<'_, RequestContext>,
) -> Response {
    let Ok(id) = Uuid::parse_str(id) else {
        return Envelope::<()>::bad_request(Some("invalid loan id".to_string()));
    };

    let mut loan = match context
        .loans
        .get(LoanSpecification::new().with_id(id))
        .await
    {
        Ok(Some(value)) => value,
        Ok(None) => return Envelope::<()>::not_found(Some("loan not found".to_string())),
        Err(error) => return Envelope::<()>::internal_error(Some(error.to_string())),
    };

    if payload.returned && !loan.is_returned() {
        loan.returned_at = Some(UtcDateTime::now());
    }

    match context.loans.update(&loan).await {
        Ok(()) => Envelope::ok(Some(loan_response(&loan))),
        Err(error) => Envelope::<()>::internal_error(Some(error.to_string())),
    }
}

pub async fn remove_loan(
    Path(id): Path<&str>,
    Context(context): Context<'_, RequestContext>,
) -> Response {
    let Ok(id) = Uuid::parse_str(id) else {
        return Envelope::<()>::bad_request(Some("invalid loan id".to_string()));
    };

    let loan = match context
        .loans
        .get(LoanSpecification::new().with_id(id))
        .await
    {
        Ok(Some(value)) => value,
        Ok(None) => return Envelope::<()>::not_found(Some("loan not found".to_string())),
        Err(error) => return Envelope::<()>::internal_error(Some(error.to_string())),
    };

    match context.loans.remove(loan).await {
        Ok(()) => Envelope::<()>::ok(None),
        Err(error) => Envelope::<()>::internal_error(Some(error.to_string())),
    }
}
