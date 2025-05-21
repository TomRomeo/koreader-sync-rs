use std::sync::Arc;
use crate::db::Database;
use chrono::{DateTime, Utc};
use poem::http::StatusCode;
use poem_openapi::payload::{Json, PlainText};
use poem_openapi::{Object, OpenApi};
use crate::api::syncs;
use crate::api::users::auth::{authenticate, AuthPassword, AuthUser};

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
    #[oai(path = "/syncs/progress", method = "put")]
    async fn update_progress(
        &self,
        username: AuthUser,
        password: AuthPassword,
        req: Json<DocumentProgress>,
    ) -> poem::Result<PlainText<String>> {
        authenticate(&*self.db, &username, &password).await?;
        println!("update progress endpoint hit");

        match self.db.update_progress(
            &username.0.key,
            &req.document,
            &req.percentage,
            &req.progress,
            &req.device,
            &req.device_id)
            .await {
            Ok(_) => Ok(PlainText("Progress updated successfully".to_string())),
            Err(err) => {
                eprintln!("Error updating progress: {}", err);
                Err(poem::Error::from_status(StatusCode::INTERNAL_SERVER_ERROR))
            }
        }
    }
}

// // -- structs --

#[derive(Debug, Object, Clone, Eq, PartialEq)]
#[derive(serde::Deserialize)]
#[derive(sqlx::FromRow)]
pub struct DocumentProgress {
    pub document: String,
    pub percentage: String,
    pub progress: String,
    pub device: String,
    pub device_id: String,

    #[serde(with = "chrono::serde::ts_seconds_option")]
    pub timestamp: Option<DateTime<Utc>>,
}

#[derive(Debug, Object, Clone, PartialEq)]
#[derive(serde::Deserialize)]
pub struct DocumentProgressResponse {
    pub document: String,
    pub percentage: String,
    pub progress: String,
    pub device: String,
    pub device_id: String,
    pub timestamp: i64,
}