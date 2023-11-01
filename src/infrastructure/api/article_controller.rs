use crate::domain::article::{Article, ArticleId};
use crate::domain::article_use_case::ArticleUseCase;
use crate::infrastructure::sqlite_repository::sqlite_article_repository::SqliteArticleRepository;

use poem::web::Data;
use poem_openapi::payload::PlainText;
use poem_openapi::{payload::Json, OpenApi};
use poem_openapi::{ApiResponse, Tags};

use poem_openapi::param::Path;
use sqlx::SqlitePool;

use super::Api;

#[derive(Tags)]
enum ApiTags {
    /// Operations about articles.
    Article,
}

#[derive(ApiResponse)]
enum GetArticleResponse {
    /// Returns when the article exists.
    #[oai(status = 200)]
    Ok(Json<Article>),
    /// Returns when the article doesn't exist.
    #[oai(status = 404)]
    NotFound(PlainText<String>),
}

#[derive(ApiResponse)]
enum CreateArticleResponse {
    /// Returns when the article is successfully created.
    #[oai(status = 200)]
    Ok(Json<ArticleId>),
    /// Returns when the request parameters are incorrect.
    #[oai(status = 400)]
    BadRequest(PlainText<String>),
}

#[OpenApi]
impl Api {
    /// Get Article
    #[oai(path = "/article/:id", method = "get", tag = "ApiTags::Article")]
    async fn get_article(
        &self,
        id: Path<ArticleId>,
        data: Data<&SqlitePool>,
    ) -> GetArticleResponse {
        let article_repository = SqliteArticleRepository::new(data.0.clone());
        let article_use_case = ArticleUseCase::new(article_repository);

        match article_use_case.get_article(id.0).await {
            Ok(article) => GetArticleResponse::Ok(Json(article)),
            Err(err) => GetArticleResponse::NotFound(PlainText(err.to_string())),
        }
    }

    /// Create Article
    #[oai(path = "/article/create", method = "post", tag = "ApiTags::Article")]
    async fn create_article(
        &self,
        article: Json<Article>,
        data: Data<&SqlitePool>,
    ) -> CreateArticleResponse {
        let article_repository = SqliteArticleRepository::new(data.0.clone());
        let article_use_case = ArticleUseCase::new(article_repository);

        match article_use_case.create_article(article.0).await {
            Ok(article_id) => CreateArticleResponse::Ok(Json(article_id)),
            Err(err) => CreateArticleResponse::BadRequest(PlainText(err.to_string())),
        }
    }
}
