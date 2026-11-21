use std::path::PathBuf;
use std::sync::Arc;

use time::UtcDateTime;
use uuid::Uuid;

use crate::contracts::specifications::{
    BookSpecification, LoanSpecification, PublisherSpecification, ReaderSpecification,
};
use crate::contracts::{
    BooksStorage, LoansStorage, PublishersStorage, ReadersStorage, StorageError,
};
use crate::contracts_impl::{
    migrations, BooksSqliteStorage, LoansSqliteStorage, ManagedConnectionPool,
    PublishersSqliteStorage, ReadersSqliteStorage,
};
use crate::entities::loan::MAX_ACTIVE_LOANS;
use crate::entities::value_objects::{
    Address, CardNumber, InventoryNumber, Name, PhoneNumber, PublicationYear,
};
use crate::entities::{Book, Loan, Publisher, Reader};

/// Временная база данных для интеграционных тестов: миграции применяются по-настоящему.
struct TestDatabase {
    pool: Arc<ManagedConnectionPool>,
    path: PathBuf,
}

impl TestDatabase {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!("library_test_{}.db", Uuid::new_v4()));
        let pool = smol::block_on(async {
            let pool = Arc::new(
                ManagedConnectionPool::initialize(path.to_string_lossy().to_string(), 2).await,
            );
            migrations::run(&pool).await.expect("миграции не применились");
            pool
        });
        Self { pool, path }
    }
}

impl Drop for TestDatabase {
    fn drop(&mut self) {
        for suffix in ["", "-wal", "-shm"] {
            let mut name = self.path.clone().into_os_string();
            name.push(suffix);
            let _ = std::fs::remove_file(PathBuf::from(name));
        }
    }
}

fn sample_publisher(name: &str) -> Publisher {
    Publisher::create_new(
        Name::new(name).unwrap(),
        Address::new("г. Москва, ул. Кирова, д. 12").unwrap(),
    )
    .unwrap()
}

fn sample_reader(card: &str, phone: &str) -> Reader {
    Reader::create_new(
        Name::new("Иванов Иван Иванович").unwrap(),
        PhoneNumber::new(phone).unwrap(),
        CardNumber::new(card).unwrap(),
    )
    .unwrap()
}

fn sample_book(publisher: &Publisher, inventory: &str) -> Book {
    Book::create_new(
        publisher,
        Name::new("Мастер и Маргарита").unwrap(),
        Name::new("М. А. Булгаков").unwrap(),
        PublicationYear::new(2019).unwrap(),
        InventoryNumber::new(inventory).unwrap(),
    )
    .unwrap()
}

#[test]
fn publisher_crud_round_trip() {
    let database = TestDatabase::new();
    smol::block_on(async {
        let storage = PublishersSqliteStorage::new(database.pool.clone());
        let publisher = sample_publisher("АСТ");
        let id = publisher.id().value();

        storage.add(publisher).await.unwrap();

        let found = storage
            .get(PublisherSpecification::new().with_id(id))
            .await
            .unwrap()
            .expect("издательство не найдено");
        assert_eq!(found.name().value(), "АСТ");
        assert_eq!(storage.list(PublisherSpecification::new()).await.unwrap().len(), 1);
        assert_eq!(storage.count(PublisherSpecification::new()).await.unwrap(), 1);

        let mut changed = found.clone();
        changed.name = Name::new("АСТ-Пресс").unwrap();
        storage.update(&changed).await.unwrap();
        let found = storage
            .get(PublisherSpecification::new().with_id(id))
            .await
            .unwrap()
            .unwrap();
        assert_eq!(found.name().value(), "АСТ-Пресс");
        assert!(found.record_lifetime().updated_at().is_some());

        storage.remove(found).await.unwrap();
        assert!(storage
            .get(PublisherSpecification::new().with_id(id))
            .await
            .unwrap()
            .is_none());
        assert_eq!(storage.count(PublisherSpecification::new()).await.unwrap(), 0);
    });
}

#[test]
fn duplicate_publisher_name_is_rejected() {
    let database = TestDatabase::new();
    smol::block_on(async {
        let storage = PublishersSqliteStorage::new(database.pool.clone());
        storage.add(sample_publisher("АСТ")).await.unwrap();
        let error = storage.add(sample_publisher("АСТ")).await.unwrap_err();
        assert!(matches!(error, StorageError::UniqueViolation(_)));
    });
}

#[test]
fn book_with_unknown_publisher_is_rejected() {
    let database = TestDatabase::new();
    smol::block_on(async {
        let storage = BooksSqliteStorage::new(database.pool.clone());
        let ghost = sample_publisher("Не сохранено");
        let error = storage.add(sample_book(&ghost, "ИНВ-0001")).await.unwrap_err();
        assert!(matches!(error, StorageError::ForeignKeyViolation(_)));
    });
}

#[test]
fn loan_lifecycle_and_availability() {
    let database = TestDatabase::new();
    smol::block_on(async {
        let publishers = PublishersSqliteStorage::new(database.pool.clone());
        let books = BooksSqliteStorage::new(database.pool.clone());
        let readers = ReadersSqliteStorage::new(database.pool.clone());
        let loans = LoansSqliteStorage::new(database.pool.clone());

        let publisher = sample_publisher("АСТ");
        publishers.add(publisher.clone()).await.unwrap();
        let book = sample_book(&publisher, "ИНВ-0001");
        books.add(book.clone()).await.unwrap();
        let reader = sample_reader("ЧБ-0001", "79001112233");
        readers.add(reader.clone()).await.unwrap();

        assert_eq!(books.count(BookSpecification::new().with_only_available()).await.unwrap(), 1);

        let loan = Loan::issue(&book, &reader, 0, false, UtcDateTime::now()).unwrap();
        loans.add(loan.clone()).await.unwrap();

        assert_eq!(loans.count_active_for_reader(reader.id().value()).await.unwrap(), 1);
        assert!(loans.is_book_issued(book.id().value()).await.unwrap());
        assert_eq!(books.count(BookSpecification::new().with_only_available()).await.unwrap(), 0);

        // повторная выдача той же книги запрещена
        let duplicate = Loan::issue(&book, &reader, 1, false, UtcDateTime::now()).unwrap();
        let error = loans.add(duplicate).await.unwrap_err();
        assert!(matches!(error, StorageError::BusinessRule(_)));

        // возврат книги
        let mut closed = loan.clone();
        closed.close(UtcDateTime::now()).unwrap();
        loans.update(&closed).await.unwrap();
        assert!(!loans.is_book_issued(book.id().value()).await.unwrap());
        assert_eq!(loans.list(LoanSpecification::new().with_only_active()).await.unwrap().len(), 0);
        assert_eq!(books.count(BookSpecification::new().with_only_available()).await.unwrap(), 1);

        // после возврата книгу можно выдать снова
        let next = Loan::issue(&book, &reader, 0, false, UtcDateTime::now()).unwrap();
        loans.add(next).await.unwrap();
    });
}

#[test]
fn reader_limit_is_enforced() {
    let database = TestDatabase::new();
    smol::block_on(async {
        let publishers = PublishersSqliteStorage::new(database.pool.clone());
        let books = BooksSqliteStorage::new(database.pool.clone());
        let readers = ReadersSqliteStorage::new(database.pool.clone());
        let loans = LoansSqliteStorage::new(database.pool.clone());

        let publisher = sample_publisher("АСТ");
        publishers.add(publisher.clone()).await.unwrap();
        let reader = sample_reader("ЧБ-0001", "79001112233");
        readers.add(reader.clone()).await.unwrap();

        for index in 0..MAX_ACTIVE_LOANS + 1 {
            let book = sample_book(&publisher, &format!("ИНВ-{index:04}"));
            books.add(book.clone()).await.unwrap();

            // В домен намеренно передаётся нулевой счётчик книг на руках.
            // Это проверяет, что адаптер самостоятельно контролирует предел
            // по фактическому состоянию базы данных, а не по данным вызывающей стороны.
            let loan = Loan::issue(&book, &reader, 0, false, UtcDateTime::now()).unwrap();
            let result = loans.add(loan).await;

            if index < MAX_ACTIVE_LOANS {
                result.unwrap();
            } else {
                assert!(matches!(result.unwrap_err(), StorageError::BusinessRule(_)));
            }
        }

        assert_eq!(
            loans.count_active_for_reader(reader.id().value()).await.unwrap(),
            MAX_ACTIVE_LOANS
        );
    });
}

#[test]
fn book_search_finds_by_title_and_author() {
    let database = TestDatabase::new();
    smol::block_on(async {
        let publishers = PublishersSqliteStorage::new(database.pool.clone());
        let books = BooksSqliteStorage::new(database.pool.clone());

        let publisher = sample_publisher("АСТ");
        publishers.add(publisher.clone()).await.unwrap();
        books.add(sample_book(&publisher, "ИНВ-0001")).await.unwrap();

        let by_title = books.list(BookSpecification::new().with_search("Мастер")).await.unwrap();
        let by_author = books.list(BookSpecification::new().with_search("Булгаков")).await.unwrap();
        let nothing = books.list(BookSpecification::new().with_search("Дюна")).await.unwrap();

        assert_eq!(by_title.len(), 1);
        assert_eq!(by_author.len(), 1);
        assert!(nothing.is_empty());
    });
}