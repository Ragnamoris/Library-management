use std::sync::Arc;

use rusqlite::{params, Connection};
use time::{Duration, UtcDateTime};
use uuid::Uuid;

use crate::contracts::StorageResult;
use crate::contracts_impl::{storage_err, ManagedConnectionPool};
use crate::utils::encode_datetime;

const LOAN_TERM_DAYS: i64 = 20;

pub async fn run(pool: &Arc<ManagedConnectionPool>) -> StorageResult<()> {
    let client = pool.acquire_connection().await?;

    let result = client
        .conn(|connection| -> rusqlite::Result<bool> {
            let staff_count: i64 =
                connection.query_row("SELECT COUNT(*) FROM staff", [], |row| row.get(0))?;
            if staff_count > 0 {
                return Ok(false);
            }

            let now = UtcDateTime::now();
                        let transaction = connection.unchecked_transaction()?;

            insert_staff(&transaction, "admin", "admin123", "Смирнова Ольга Петровна", "admin", now)?;
            insert_staff(&transaction, "librarian", "lib123", "Иванов Иван Иванович", "librarian", now)?;

            let ast = insert_publisher(&transaction, "АСТ", "г. Москва, ул. Кирова, д. 12", now)?;
            let eksmo = insert_publisher(&transaction, "Эксмо", "г. Москва, Ленинградский пр., д. 78", now)?;
            let piter = insert_publisher(&transaction, "Питер", "г. Санкт-Петербург, Невский пр., д. 45", now)?;
            let bhv = insert_publisher(&transaction, "БХВ-Петербург", "г. Санкт-Петербург, Лиговский пр., д. 30", now)?;

            let b1 = insert_book(&transaction, ast, "Мастер и Маргарита", "М. А. Булгаков", 1967, "ИНВ-0001", now - Duration::days(120))?;
            let b2 = insert_book(&transaction, ast, "Преступление и наказание", "Ф. М. Достоевский", 1866, "ИНВ-0002", now - Duration::days(120))?;
            let b3 = insert_book(&transaction, ast, "Три товарища", "Э. М. Ремарк", 1936, "ИНВ-0003", now - Duration::days(100))?;
            let b4 = insert_book(&transaction, piter, "Чистый код", "Р. Мартин", 2009, "ИНВ-0004", now - Duration::days(90))?;
            let b5 = insert_book(&transaction, piter, "Совершенный код", "С. Макконнелл", 2004, "ИНВ-0005", now - Duration::days(90))?;
            let _b6 = insert_book(&transaction, bhv, "Язык программирования Rust", "С. Клабник, К. Николс", 2019, "ИНВ-0006", now - Duration::days(60))?;
            let _b7 = insert_book(&transaction, bhv, "Алгоритмы. Построение и анализ", "Т. Кормен", 2013, "ИНВ-0007", now - Duration::days(60))?;
            let b8 = insert_book(&transaction, eksmo, "Гарри Поттер и философский камень", "Дж. К. Роулинг", 1997, "ИНВ-0008", now - Duration::days(45))?;
            let b9 = insert_book(&transaction, eksmo, "Шантарам", "Г. Д. Робертс", 2003, "ИНВ-0009", now - Duration::days(45))?;
            // Книга в фонде, но ещё ни разу не выдавалась:
            let _b10 = insert_book(&transaction, eksmo, "Дюна", "Ф. Герберт", 1965, "ИНВ-0010", now - Duration::days(10))?;

            let r1 = insert_reader(&transaction, "Петров Пётр Петрович", "+7 (900) 111-22-33", "ЧБ-0001", now - Duration::days(150))?;
            let r2 = insert_reader(&transaction, "Сидорова Анна Ивановна", "+7 (900) 444-55-66", "ЧБ-0002", now - Duration::days(120))?;
            let r3 = insert_reader(&transaction, "Кузнецов Дмитрий Сергеевич", "+7 (900) 777-88-99", "ЧБ-0003", now - Duration::days(90))?;
            let r4 = insert_reader(&transaction, "Волкова Мария Андреевна", "+7 (900) 123-45-67", "ЧБ-0004", now - Duration::days(30))?;

            insert_loan(&transaction, b1, r1, now - Duration::days(30), Some(now - Duration::days(15)))?;
            insert_loan(&transaction, b2, r1, now - Duration::days(25), Some(now - Duration::days(2)))?;
            insert_loan(&transaction, b5, r2, now - Duration::days(10), Some(now - Duration::days(8)))?;
            insert_loan(&transaction, b9, r3, now - Duration::days(35), Some(now - Duration::days(30)))?;
            insert_loan(&transaction, b1, r2, now - Duration::days(5), None)?;
            insert_loan(&transaction, b3, r3, now - Duration::days(40), None)?;
            insert_loan(&transaction, b4, r1, now - Duration::days(3), None)?;
            insert_loan(&transaction, b8, r4, now, None)?;

            transaction.commit()?;
            Ok(true)
        })
        .await;

    pool.return_connection(client).await?;
    let inserted = result.map_err(storage_err)?;

    if inserted {
        println!("Демо-данные загружены: 2 работника, 4 издательства, 10 книг, 4 читателя, 8 выдач.");
        println!("Учётные записи: admin / admin123 (администратор), librarian / lib123 (библиотекарь)");
    } else {
        println!("Демо-данные уже загружены, повторное заполнение не требуется.");
    }
    Ok(())
}

fn insert_staff(
    connection: &Connection,
    login: &str,
    password: &str,
    full_name: &str,
    role: &str,
    created_at: UtcDateTime,
) -> rusqlite::Result<Uuid> {
    let id = Uuid::new_v4();
    let password_hash = bcrypt::hash(password, bcrypt::DEFAULT_COST).expect("password hashing failed");
    connection.execute(
        "INSERT INTO staff(id, login, password_hash, full_name, role, created_at, updated_at, deleted_at)
         VALUES(?1, ?2, ?3, ?4, ?5, ?6, NULL, NULL)",
        params![id, login, password_hash, full_name, role, encode_datetime(created_at)],
    )?;
    Ok(id)
}

fn insert_publisher(
    connection: &Connection,
    name: &str,
    address: &str,
    created_at: UtcDateTime,
) -> rusqlite::Result<Uuid> {
    let id = Uuid::new_v4();
    connection.execute(
        "INSERT INTO publishers(id, name, address, created_at, updated_at, deleted_at)
         VALUES(?1, ?2, ?3, ?4, NULL, NULL)",
        params![id, name, address, encode_datetime(created_at)],
    )?;
    Ok(id)
}

fn insert_book(
    connection: &Connection,
    publisher_id: Uuid,
    title: &str,
    author: &str,
    publication_year: i32,
    inventory_number: &str,
    created_at: UtcDateTime,
) -> rusqlite::Result<Uuid> {
    let id = Uuid::new_v4();
    connection.execute(
        "INSERT INTO books(id, publisher_id, title, author, publication_year, inventory_number, created_at, updated_at, deleted_at)
         VALUES(?1, ?2, ?3, ?4, ?5, ?6, ?7, NULL, NULL)",
        params![id, publisher_id, title, author, publication_year, inventory_number, encode_datetime(created_at)],
    )?;
    Ok(id)
}

fn insert_reader(
    connection: &Connection,
    full_name: &str,
    phone_number: &str,
    card_number: &str,
    created_at: UtcDateTime,
) -> rusqlite::Result<Uuid> {
    let id = Uuid::new_v4();
    connection.execute(
        "INSERT INTO readers(id, full_name, phone_number, card_number, created_at, updated_at, deleted_at)
         VALUES(?1, ?2, ?3, ?4, ?5, NULL, NULL)",
        params![id, full_name, phone_number, card_number, encode_datetime(created_at)],
    )?;
    Ok(id)
}

fn insert_loan(
    connection: &Connection,
    book_id: Uuid,
    reader_id: Uuid,
    issued_at: UtcDateTime,
    returned_at: Option<UtcDateTime>,
) -> rusqlite::Result<Uuid> {
    let id = Uuid::new_v4();
    let due_at = issued_at + Duration::days(LOAN_TERM_DAYS);
    connection.execute(
        "INSERT INTO loans(id, book_id, reader_id, issued_at, due_at, returned_at, created_at, updated_at, deleted_at)
         VALUES(?1, ?2, ?3, ?4, ?5, ?6, ?7, NULL, NULL)",
        params![
            id,
            book_id,
            reader_id,
            encode_datetime(issued_at),
            encode_datetime(due_at),
            returned_at.map(encode_datetime),
            encode_datetime(issued_at)
        ],
    )?;
    Ok(id)
}