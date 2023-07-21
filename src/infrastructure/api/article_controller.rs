use crate::domain::article::Article;
use crate::domain::article_use_case::ArticleUseCase;
use crate::infrastructure::fake_article_repository::FakeArticleRepository;

use poem_openapi::ApiResponse;
use poem_openapi::{payload::Json, OpenApi};

use poem_openapi::param::Path;

use super::Api;

#[derive(ApiResponse)]
enum ArticleResponse {
    /// Returns when the article exists.
    #[oai(status = 200)]
    Ok(Json<Article>),
    /// Returns when the article doesn't exist.
    #[oai(status = 404)]
    NotFound,
}

#[OpenApi]
impl Api {
    /// GET Article
    #[oai(path = "/article/:id", method = "get")]
    async fn get_article(&self, id: Path<i32>) -> ArticleResponse {
        let fake_article_repository = FakeArticleRepository;
        let article_use_case = ArticleUseCase::new(fake_article_repository);

        match article_use_case.get(id.0) {
            Ok(article) => ArticleResponse::Ok(Json(article)),
            Err(_) => ArticleResponse::NotFound,
        }
    }
}
