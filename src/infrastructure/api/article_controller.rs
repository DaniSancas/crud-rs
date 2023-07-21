
use crate::domain::article_use_case::ArticleUseCase;
use crate::infrastructure::fake_article_repository::FakeArticleRepository;


use poem_openapi::{payload::PlainText, OpenApi};

use poem_openapi::param::Path;

use super::Api;

#[OpenApi]
impl Api {
    /// GET Article
    #[oai(path = "/article/:id", method = "get")]
    async fn get_article(&self, id: Path<i32>) -> PlainText<String> {
        let fake_article_repository = FakeArticleRepository;
        let article_use_case = ArticleUseCase::new(fake_article_repository);

        PlainText(format!("{:?}", article_use_case.get(id.0)))
    }
}
