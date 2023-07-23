mod application;
mod domain;
mod infrastructure;

use dotenv::dotenv;
use infrastructure::{api::Api, sqlite_repository::SQLiteRepository};
use poem::{listener::TcpListener, EndpointExt, Route, Server};
use poem_openapi::OpenApiService;

#[tokio::main]
async fn main() {
    // Load ENV variables
    dotenv().ok();
    let db_path = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file.");

    // Init DB connection
    let db = SQLiteRepository::new(db_path.as_str())
        .await
        .expect("Unable to generate a DB connection.");

    // Init web app
    let api_service =
        OpenApiService::new(Api, "Article CRUD", "1.0").server("http://localhost:3000");
    let ui = api_service.swagger_ui();
    let app = Route::new()
        .nest("/", api_service)
        .nest("/docs", ui)
        .data(db.connection_pool);

    // Run server
    let _ = Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await;
}
