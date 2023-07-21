use crate::application::article_repository::ArticleRepository;
use crate::domain::article::Article;

pub struct ArticleUseCase<T: ArticleRepository> {
    article_repository: T,
}

impl<T: ArticleRepository> ArticleUseCase<T> {
    pub fn new(article_repository: T) -> Self {
        ArticleUseCase { article_repository }
    }

    pub fn get(&self, id: i32) -> Article {
        self.article_repository.get(id)
    }
}
