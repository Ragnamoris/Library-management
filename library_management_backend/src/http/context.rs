use std::sync::Arc;

use crate::contracts_impl::{
    BooksSqliteStorage, LoansSqliteStorage, PublishersSqliteStorage, ReadersSqliteStorage,
};

#[derive(Clone)]
pub struct RequestContext {
    pub publishers: Arc<PublishersSqliteStorage>,
    pub readers: Arc<ReadersSqliteStorage>,
    pub books: Arc<BooksSqliteStorage>,
    pub loans: Arc<LoansSqliteStorage>,
}

impl RequestContext {
    pub fn new(
        publishers: Arc<PublishersSqliteStorage>,
        readers: Arc<ReadersSqliteStorage>,
        books: Arc<BooksSqliteStorage>,
        loans: Arc<LoansSqliteStorage>,
    ) -> Self {
        Self {
            publishers,
            readers,
            books,
            loans,
        }
    }
}
