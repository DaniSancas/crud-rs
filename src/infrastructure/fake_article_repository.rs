use crate::{application::article_repository::ArticleRepository, domain::article::Article};

use eyre::Result;

pub struct FakeArticleRepository;

impl ArticleRepository for FakeArticleRepository {
    fn get(&self, id: i32) -> Result<Article> {
        Ok(Article {
            id,
            title: "First article".to_string(),
            content: "The body of the article...".to_string(),
        })
    }
}
