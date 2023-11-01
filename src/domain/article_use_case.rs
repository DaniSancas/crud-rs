use crate::application::article_repository::ArticleRepository;
use crate::domain::article::Article;
use crate::domain::article::ArticleId;

use eyre::Result;

pub struct ArticleUseCase<T: ArticleRepository> {
    article_repository: T,
}

impl<T: ArticleRepository> ArticleUseCase<T> {
    pub const fn new(article_repository: T) -> Self {
        Self { article_repository }
    }

    pub async fn get_article(&self, id: ArticleId) -> Result<Article> {
        self.article_repository.get_article(id).await
    }

    pub async fn create_article(&self, article: Article) -> Result<ArticleId> {
        self.article_repository.create_article(article).await
    }
}
