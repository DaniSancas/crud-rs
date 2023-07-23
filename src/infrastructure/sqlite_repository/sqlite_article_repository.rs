use crate::{application::article_repository::ArticleRepository, domain::article::Article};
use sqlx::SqlitePool;

use async_trait::async_trait;
use eyre::Result;

pub struct SqliteArticleRepository {
    connection_pool: SqlitePool,
}

impl SqliteArticleRepository {
    pub fn new(connection_pool: SqlitePool) -> Self {
        SqliteArticleRepository { connection_pool }
    }
}

#[async_trait]
impl ArticleRepository for SqliteArticleRepository {
    async fn get(&self, id: i64) -> Result<Article> {
        Ok(sqlx::query_as!(
            Article,
            "select id, title, content from articles where id = ?",
            id
        )
        .fetch_one(&self.connection_pool)
        .await?)
    }
}
