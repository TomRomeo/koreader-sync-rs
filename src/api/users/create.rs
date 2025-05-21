use std::sync::Arc;
use poem_openapi::payload::{Json, PlainText};
use poem_openapi::{ApiResponse, Object, OpenApi};
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
    #[oai(path = "/users/create", method = "post")]
    async fn create_user(&self, req: Json<CreateUserRequest>) -> CreateUserResponse {
        println!("register endpoint hit");

        let hashed_password = match bcrypt::hash(&req.password, bcrypt::DEFAULT_COST) {
            Ok(hash) => hash,
            Err(err) => {
                eprint!("Failed to hash password: {}", err);
                return CreateUserResponse::ServerError(PlainText("Failed to create user".to_string()));
            }
        };

        match self.db.create_user(&req.username, &hashed_password).await {
            Ok(..) => CreateUserResponse::Success(PlainText("User created successfully".to_string())),
            Err(err) => {
                eprint!("Failed to create user: {}", err);
                CreateUserResponse::Failure(PlainText("Failed to create user".to_string()))
            }
        }
    }
}

// -- structs --

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
    #[oai(status = 500)]
    ServerError(PlainText<String>),
}
