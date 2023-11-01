use crate::{
    application::article_repository::ArticleRepository,
    domain::article::{Article, ArticleId},
};
use sqlx::SqlitePool;

use async_trait::async_trait;
use eyre::Result;

pub struct SqliteArticleRepository(pub SqlitePool);

#[async_trait]
impl ArticleRepository for SqliteArticleRepository {
    async fn get_article(&self, id: ArticleId) -> Result<Article> {
        Ok(sqlx::query_as!(
            Article,
            "select id, title, content from articles where id = ?",
            id
        )
        .fetch_one(&self.0)
        .await?)
    }

    async fn create_article(&self, article: Article) -> Result<ArticleId> {
        Ok(sqlx::query!(
            "insert into articles (title, content) values (?, ?)",
            article.title,
            article.content
        )
        .execute(&self.0)
        .await?
        .last_insert_rowid())
    }
}
