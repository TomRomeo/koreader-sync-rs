use crate::api::syncs::update_progress::DocumentProgress;
use crate::db::Database;
use poem::Result;
use poem_openapi::auth::Basic;
use poem_openapi::param::Path;
use poem_openapi::payload::{Json, PlainText};
use poem_openapi::{OpenApi, SecurityScheme};
use std::sync::Arc;

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

#[derive(SecurityScheme)]
#[oai(ty = "basic")]
pub struct BasicAuthentication(Basic);

#[OpenApi]
impl Api {

    /// Health Check
    #[oai(path = "/healthcheck", method = "get")]
    async fn healthcheck(&self) -> PlainText<&'static str> {
        healthcheck::handler()
    }

    /// Get progress
    #[oai(path = "/syncs/progress/:id", method = "get")]
    async fn get_progress(&self, auth: BasicAuthentication, id: Path<String>) -> Result<Json<DocumentProgress>> {
        let (username, _) = self.auth(auth).await?;
        Ok(syncs::get_progress::handler(self.db.as_ref(), username, id.0).await?)
    }

    /// Update progress
    #[oai(path = "/syncs/progress", method = "post")]
    async fn update_progress(&self, auth: BasicAuthentication, req: Json<DocumentProgress>) -> Result<PlainText<String>> {
        let (username, _) = self.auth(auth).await?;
        Ok(syncs::update_progress::handler(self.db.as_ref(),
            &username,
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
        users::create::handler(self.db.as_ref(), &req.username, &req.password).await
    }

}
