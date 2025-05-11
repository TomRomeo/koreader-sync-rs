use async_trait::async_trait;
use crate::api::syncs::update_progress::DocumentProgress;

pub mod postgres;

#[async_trait]
pub trait Database: Send + Sync {
    async fn get_progress(&self, username: &str, id: &str) -> Result<DocumentProgress, sqlx::Error>;
    async fn update_progress(&self, username: &str, document: &str, percentage: &str, progress: &str, device: &str, device_id: &str) -> Result<(), sqlx::Error>;
    async fn create_user(&self, username: &str, password: &str) -> Result<(), sqlx::Error>;
    async fn validate_password(&self, username: &str, password: &str) -> Result<bool, sqlx::Error>;
}