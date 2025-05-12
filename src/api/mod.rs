use crate::api::syncs::update_progress::DocumentProgress;
use crate::db::Database;
use poem::Result;
use poem_openapi::param::Path;
use poem_openapi::payload::{Json, PlainText};
use poem_openapi::{OpenApi};
use std::sync::Arc;
use crate::api::users::auth::{ApiAuthentication};

mod healthcheck;
pub mod syncs;
mod users;

pub struct Api {
    db: Arc<dyn Database>
}

impl Api {
    pub fn new(db: Arc<dyn Database>) -> Self {
        Self { db }
    }
}


#[OpenApi]
impl Api {

    /// Health Check
    #[oai(path = "/healthcheck", method = "get")]
    async fn healthcheck(&self) -> PlainText<&'static str> {
        healthcheck::handler()
    }

    /// Get progress
    #[oai(path = "/syncs/progress/:id", method = "get")]
    async fn get_progress(&self, auth: ApiAuthentication, id: Path<String>) -> Result<Json<DocumentProgress>> {
        self.auth(&auth).await?;
        Ok(syncs::get_progress::handler(self.db.as_ref(), auth.username, id.0).await?)
    }

    /// Update progress
    #[oai(path = "/syncs/progress", method = "post")]
    async fn update_progress(&self, auth: ApiAuthentication, req: Json<DocumentProgress>) -> Result<PlainText<String>> {
        self.auth(&auth).await?;
        Ok(syncs::update_progress::handler(self.db.as_ref(),
            &auth.username,
            &req.document,
            &req.percentage,
            &req.progress,
            &req.device,
            &req.device_id)
        .await)
    }

    /// Create user
    #[oai(path = "/users/create", method = "post")]
    async fn create_user(&self, req: Json<users::create::CreateUserRequest>) -> PlainText<String> {
        println!("register endpoint hit");
        users::create::handler(self.db.as_ref(), &req.username, &req.password).await
    }

    // Auth user
    #[oai(path = "/users/auth", method = "get")]
    async fn auth_user(&self, auth: ApiAuthentication) -> Result<PlainText<String>> {
        println!("auth endpoint hit");
        Ok(users::auth::handler(self.db.as_ref(), &auth.username, &auth.password).await?)
    }

}
