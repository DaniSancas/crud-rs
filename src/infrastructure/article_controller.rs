use crate::domain::article::Article;
use crate::domain::article_use_case::ArticleUseCase;
use crate::infrastructure::fake_article_repository::FakeArticleRepository;

pub fn get_article(id: i32) -> Article {
    let fake_article_repository = FakeArticleRepository;
    let article_use_case = ArticleUseCase::new(fake_article_repository);
    article_use_case.get(id)
}
