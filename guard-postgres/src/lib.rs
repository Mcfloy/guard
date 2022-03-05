use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;

mod namespace;
mod permission;

pub struct PostgresRepository {
    pool: Pool<Postgres>
}

impl PostgresRepository {
    pub async fn new() -> Self {
        let db_uri = std::env::var("DATABASE_URL")
            .unwrap_or("postgres://localhost:5432/postgres".to_string());

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&db_uri)
            .await
            .unwrap();

        PostgresRepository {
            pool
        }
    }
}
