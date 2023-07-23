use crate::domain::article::Article;
use async_trait::async_trait;
use eyre::Result;

#[async_trait]
pub trait ArticleRepository {
    async fn get_article(&self, id: i64) -> Result<Article>;
}
