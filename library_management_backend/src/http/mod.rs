pub mod books;
pub mod context;
pub mod envelope;
pub mod loans;
pub mod publishers;
pub mod readers;

use std::sync::Arc;

use ohkami::fang::Context;
use ohkami::{Ohkami, Route};

use crate::contracts_impl::{
    BooksSqliteStorage, LoansSqliteStorage, ManagedConnectionPool, PublishersSqliteStorage,
    ReadersSqliteStorage,
};
use crate::http::context::RequestContext;

pub fn root(pool: Arc<ManagedConnectionPool>) -> Ohkami {
    let request_context = RequestContext::new(
        Arc::new(PublishersSqliteStorage::new(pool.clone())),
        Arc::new(ReadersSqliteStorage::new(pool.clone())),
        Arc::new(BooksSqliteStorage::new(pool.clone())),
        Arc::new(LoansSqliteStorage::new(pool.clone())),
    );

    Ohkami::new(
        "/api".By(Ohkami::new((
            "/publishers".By(Ohkami::new((
                Context::new(request_context.clone()),
                "/".POST(publishers::add_publisher),
                "/:id"
                    .GET(publishers::get_publisher)
                    .PUT(publishers::update_publisher)
                    .DELETE(publishers::remove_publisher),
            ))),
            "/readers".By(Ohkami::new((
                Context::new(request_context.clone()),
                "/".POST(readers::add_reader),
                "/:id"
                    .GET(readers::get_reader)
                    .PUT(readers::update_reader)
                    .DELETE(readers::remove_reader),
            ))),
            "/books".By(Ohkami::new((
                Context::new(request_context.clone()),
                "/".POST(books::add_book),
                "/:id"
                    .GET(books::get_book)
                    .PUT(books::update_book)
                    .DELETE(books::remove_book),
            ))),
            "/loans".By(Ohkami::new((
                Context::new(request_context),
                "/".POST(loans::add_loan),
                "/:id"
                    .GET(loans::get_loan)
                    .PUT(loans::update_loan)
                    .DELETE(loans::remove_loan),
            ))),
        ))),
    )
}
