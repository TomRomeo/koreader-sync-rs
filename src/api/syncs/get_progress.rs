use crate::api::syncs::update_progress::DocumentProgress;
use poem::http::StatusCode;
use poem::{Error, Result};
use poem_openapi::payload::Json;

pub async fn handler(db: & dyn crate::db::Database, username: String, id: String) -> Result<Json<DocumentProgress>> {
    match db.get_progress(&username, &id).await {
        Ok(progress) => {
            Ok(Json(progress))
        },
        Err(err) => {
            eprintln!("Error fetching progress: {}", err);
            Err(Error::from_status(StatusCode::NOT_FOUND))
        }
    }
}
