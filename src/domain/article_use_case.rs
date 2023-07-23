use crate::application::article_repository::ArticleRepository;
use crate::domain::article::Article;

use eyre::Result;

pub struct ArticleUseCase<T: ArticleRepository> {
    article_repository: T,
}

impl<T: ArticleRepository> ArticleUseCase<T> {
    pub const fn new(article_repository: T) -> Self {
        Self { article_repository }
    }

    pub async fn get_article(&self, id: i64) -> Result<Article> {
        self.article_repository.get_article(id).await
    }
}
