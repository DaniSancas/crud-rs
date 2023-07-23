use crate::domain::article::Article;
use crate::domain::article_use_case::ArticleUseCase;
use crate::infrastructure::sqlite_repository::sqlite_article_repository::SqliteArticleRepository;

use poem::web::Data;
use poem_openapi::ApiResponse;
use poem_openapi::{payload::Json, OpenApi};

use poem_openapi::param::Path;
use sqlx::SqlitePool;

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
    async fn get_article(&self, id: Path<i64>, data: Data<&SqlitePool>) -> ArticleResponse {
        let article_repository = SqliteArticleRepository::new(data.0.clone());
        let article_use_case = ArticleUseCase::new(article_repository);

        match article_use_case.get(id.0).await {
            Ok(article) => ArticleResponse::Ok(Json(article)),
            Err(_) => ArticleResponse::NotFound,
        }
    }
}
