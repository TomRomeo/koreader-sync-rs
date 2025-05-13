mod api;
mod db;

use crate::api::Api;
use crate::db::postgres::PostgresDB;
use dotenvy::dotenv;
use poem::{listener::TcpListener, Route, Server};
use poem_openapi::OpenApiService;
use std::env;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenv().ok();

    // Get the PostgreSQL connection string from the environment variable
    let postgres_con_string = env::var("POSTGRES_CON_STRING")
        .expect("POSTGRES_CON_STRING environment variable is not set");

    let db = PostgresDB::new(&postgres_con_string)
        .await
        .expect("could not connect to db");
    
    // db.migrate().await.expect("could not migrate db");

    let api = Api::new(Arc::new(db));
    let api_service =
        OpenApiService::new(api, "Hello World", "1.0").server("http://localhost:3000");
    let ui = api_service.swagger_ui();
    let app = Route::new().nest("/", api_service).nest("/docs", ui);

    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await;
}
