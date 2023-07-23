use crate::{application::article_repository::ArticleRepository, domain::article::Article};

use async_trait::async_trait;
use eyre::Result;

pub struct FakeArticleRepository;

#[async_trait]
impl ArticleRepository for FakeArticleRepository {
    async fn get(&self, id: i64) -> Result<Article> {
        Ok(Article {
            id,
            title: "First article".to_string(),
            content: "The body of the article...".to_string(),
        })
    }
}
