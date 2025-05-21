mod api;
mod db;

use crate::api::{healthcheck, syncs::get_progress, users};
use crate::db::postgres::PostgresDB;
use dotenvy::dotenv;
use poem::{listener::TcpListener, Route, Server};
use poem_openapi::OpenApiService;
use std::env;
use std::sync::Arc;
use crate::api::syncs::update_progress;
use crate::db::Database;

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

    let db_arc: Arc<dyn Database> = Arc::new(db);
    let health_check_handler = healthcheck::HealthCheckHandler::new(Arc::clone(&db_arc));
    let get_progress_handler = get_progress::Handler::new(Arc::clone(&db_arc));
    let update_progress_handler = update_progress::Handler::new(Arc::clone(&db_arc));
    let auth_user_handler = users::auth::Handler::new(Arc::clone(&db_arc));
    let create_user_handler = users::create::Handler::new(Arc::clone(&db_arc));
    let api_service = OpenApiService::new((
        auth_user_handler,
        create_user_handler,
        health_check_handler,
        get_progress_handler,
        update_progress_handler
    ), "KOReader Sync Server written in Rust", "1.0").server("http://localhost:3000");
    let ui = api_service.swagger_ui();
    let app = Route::new().nest("/", api_service).nest("/docs", ui);

    let listen_address = env::var("LISTEN_ADDRESS").unwrap_or(String::from("0.0.0.0:3000"));

    println!("Listening on {}", listen_address);
    Server::new(TcpListener::bind(listen_address))
        .run(app)
        .await;
}
