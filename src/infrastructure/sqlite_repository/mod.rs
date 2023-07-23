use eyre::Result;
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

pub struct SQLiteRepository {
    pub connection_pool: SqlitePool,
}

impl SQLiteRepository {
    pub async fn new(db_path: &str) -> Result<Self> {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(db_path)
            .await?;

        Ok(SQLiteRepository {
            connection_pool: pool,
        })
    }
}

pub mod sqlite_article_repository;
