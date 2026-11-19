use async_sqlite::{Client, ClientBuilder};

use crate::contracts::StorageError;
use crate::contracts_impl::storage_err;

type ManagedPool = (
    smol::channel::Sender<Client>,
    smol::channel::Receiver<Client>,
);

#[derive(Clone)]
pub struct ManagedConnectionPool {
    database_path: String,
    max_connections_count: usize,
    pool: ManagedPool,
}

impl ManagedConnectionPool {
    pub async fn initialize(database_path: String, max_connections_count: usize) -> Self {
        let (sender, receiver) = smol::channel::bounded::<Client>(max_connections_count);

        for _ in 0..max_connections_count {
            let connection = Self::create_connection(database_path.clone()).await;
            sender
                .send(connection)
                .await
                .expect("cannot fill the connection pool");
        }

        Self {
            database_path,
            max_connections_count,
            pool: (sender, receiver),
        }
    }

    async fn create_connection(database_path: String) -> Client {
        let builder: ClientBuilder = ClientBuilder::new().path(database_path);
        let client = builder
            .open()
            .await
            .expect("cannot open a database connection");

        client
            .conn(|connection| {
                connection.execute_batch(
                    "PRAGMA foreign_keys = ON;
                     PRAGMA journal_mode = WAL;
                     PRAGMA busy_timeout = 5000;",
                )
            })
            .await
            .expect("cannot configure a database connection");

        client
    }

    pub async fn acquire_connection(&self) -> Result<Client, StorageError> {
        let connection = self.pool.1.recv().await.map_err(storage_err)?;
        Ok(connection)
    }

    pub async fn return_connection(&self, connection: Client) -> Result<(), StorageError> {
        self.pool.0.send(connection).await.map_err(storage_err)?;
        Ok(())
    }

    pub fn database_path(&self) -> &str {
        &self.database_path
    }

    pub fn max_connections_count(&self) -> usize {
        self.max_connections_count
    }
}