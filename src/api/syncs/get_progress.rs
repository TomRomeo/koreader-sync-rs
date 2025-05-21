use std::sync::Arc;
use crate::api::syncs::update_progress::DocumentProgress;
use poem::http::StatusCode;
use poem::{Error, Result};
use poem_openapi::OpenApi;
use poem_openapi::param::Path;
use poem_openapi::payload::{Json, PlainText};
use crate::api::users::auth::{authenticate, AuthPassword, AuthUser};
use crate::db::Database;

pub struct Handler {
    db: Arc<dyn Database>,
}

impl Handler {
    pub fn new(db: Arc<dyn Database>) -> Self {
        Self { db }
    }
}

#[OpenApi]
impl Handler {
    #[oai(path = "/syncs/progress/:id", method = "get")]
    async fn get_progress(
        &self,
        username: AuthUser,
        password: AuthPassword,
        id: Path<String>,
    ) -> Result<Json<DocumentProgress>> {
        authenticate(&*self.db, &username, &password).await?;

        match self.db.get_progress(&username.0.key, &id).await {
            Ok(progress) => {
                Ok(Json(progress))
            },
            Err(err) => {
                eprintln!("Error fetching progress: {}", err);
                Err(Error::from_status(StatusCode::NOT_FOUND))
            }
        }
        
    }
}