mod application;
mod domain;
mod infrastructure;

use infrastructure::api::Api;
use poem::{listener::TcpListener, Route, Server};
use poem_openapi::OpenApiService;

#[tokio::main]
async fn main() {
    let api_service =
        OpenApiService::new(Api, "Article CRUD", "1.0").server("http://localhost:3000");
    let ui = api_service.swagger_ui();
    let app = Route::new().nest("/", api_service).nest("/docs", ui);

    let _ = Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await;
}
