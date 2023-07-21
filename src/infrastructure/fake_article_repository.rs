use crate::{application::article_repository::ArticleRepository, domain::article::Article};

pub struct FakeArticleRepository;

impl ArticleRepository for FakeArticleRepository {
    fn get(&self, id: i32) -> crate::domain::article::Article {
        Article {
            id,
            title: "First article".to_string(),
            content: "The body of the article...".to_string(),
        }
    }
}
