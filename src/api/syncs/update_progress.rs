use crate::db::Database;
use chrono::{DateTime, Utc};
use poem_openapi::payload::PlainText;
use poem_openapi::Object;

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

pub async fn handler(db: & dyn Database, username: &str, document: &str, percentage: &str, progress: &str, device: &str, device_id: &str ) -> PlainText<String> {
    match db.update_progress(username, document, percentage, progress, device, device_id).await {
        Ok(_) => PlainText("Progress updated successfully".to_string()),
        Err(err) => {
            eprintln!("Error updating progress: {}", err);
            PlainText("Error updating progress".to_string())
        }
    }
}