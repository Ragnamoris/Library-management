// Доменный слой и слой хранения реализованы на этапах 4–5 и подключаются
// к обработчикам команд и запросов на этапе 6.
// TODO(этап 6): снять атрибуты после подключения обработчиков use-case.
#[allow(dead_code, unused_imports)]
mod contracts;
#[allow(dead_code, unused_imports)]
mod contracts_impl;
#[allow(dead_code, unused_imports)]
mod entities;
mod http;
mod seed;
mod utils;

use std::sync::Arc;

use contracts_impl::{migrations, ManagedConnectionPool};
use http::root;

fn main() -> smol::io::Result<()> {
    smol::block_on(async {
        let database_path =
            std::env::var("DATABASE_PATH").unwrap_or_else(|_| "databases/data.db".to_string());
        let command = std::env::args().nth(1).unwrap_or_else(|| "serve".to_string());

        let pool = ManagedConnectionPool::initialize(database_path, 10).await;
        println!(
            "database: {}, connections: {}",
            pool.database_path(),
            pool.max_connections_count()
        );
        let pool = Arc::new(pool);

        println!("Применение миграций:");
        let applied = migrations::run(&pool)
            .await
            .expect("не удалось применить миграции");
        println!("Новых миграций применено: {applied}");

        match command.as_str() {
            "migrate" => {
                migrations::report(&pool)
                    .await
                    .expect("ошибка чтения схемы базы данных");
            }
            "seed" => {
                seed::run(&pool)
                    .await
                    .expect("не удалось загрузить демо-данные");
                migrations::report(&pool)
                    .await
                    .expect("ошибка чтения схемы базы данных");
            }
            "serve" => {
                println!("HTTP-сервер: http://localhost:3000/api");
                root(pool).howl("localhost:3000").await;
            }
            other => {
                println!("Неизвестная команда: {other}. Доступно: serve | migrate | seed");
            }
        }

        Ok(())
    })
}