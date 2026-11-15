mod contracts;
mod contracts_impl;
mod entities;
mod http;
mod utils;

use std::sync::Arc;

use contracts_impl::ManagedConnectionPool;
use http::root;

fn main() -> smol::io::Result<()> {
    smol::block_on(async {
        let pool = ManagedConnectionPool::initialize("databases/data.db".to_string(), 10).await;
        println!(
            "database: {}, connections: {}",
            pool.database_path(),
            pool.max_connections_count()
        );
        let pool = Arc::new(pool);

        root(pool).howl("localhost:3000").await;

        Ok(())
    })
}
