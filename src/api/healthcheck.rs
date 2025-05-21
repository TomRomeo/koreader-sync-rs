use std::sync::Arc;
use poem_openapi::{ApiResponse, OpenApi};
use poem_openapi::payload::PlainText;
use crate::db::Database;

pub struct HealthCheckHandler {
    db: Arc<dyn Database>,
}

impl HealthCheckHandler {
    pub fn new(db: Arc<dyn Database>) -> Self {
        Self { db }
    }
}

#[OpenApi]
impl HealthCheckHandler {
    #[oai(path = "/healthcheck", method = "get")]
    async fn healthcheck(&self) -> HealthCheckResponse {
        HealthCheckResponse::Ok(PlainText("OK"))
    }
}

#[derive(ApiResponse)]
pub enum HealthCheckResponse {
    #[oai(status = 200)]
    Ok(PlainText<&'static str>),
}
