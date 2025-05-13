use poem_openapi::payload::PlainText;
use poem_openapi::{ApiResponse, Object};

#[derive(Debug, Object, Clone, Eq, PartialEq)]
#[derive(serde::Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
}

#[derive(ApiResponse)]
pub enum CreateUserResponse {
    #[oai(status = 201)]
    Success(PlainText<String>),
    #[oai(status = 404)]
    Failure(PlainText<String>),
}

pub async fn handler(db: & dyn crate::db::Database, username: &str, password: &str) -> CreateUserResponse {
    
    match db.create_user(username, password).await {
        Ok(..) => CreateUserResponse::Success(PlainText("User created successfully".to_string())),
        Err(err) => {
            eprint!("Failed to create user: {}", err);
            CreateUserResponse::Failure(PlainText("Failed to create user".to_string()))
        }
    }
}
