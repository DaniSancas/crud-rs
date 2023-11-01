use crate::domain::article::Article;
use crate::domain::article::ArticleId;
use async_trait::async_trait;
use eyre::Result;

#[async_trait]
pub trait ArticleRepository {
    async fn get_article(&self, id: ArticleId) -> Result<Article>;

    async fn create_article(&self, article: Article) -> Result<ArticleId>;
}
