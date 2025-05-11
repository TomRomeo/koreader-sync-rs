use poem_openapi::payload::PlainText;
use poem_openapi::Object;

#[derive(Debug, Object, Clone, Eq, PartialEq)]
#[derive(serde::Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
}

pub async fn handler(db: & dyn crate::db::Database, username: &str, password: &str) -> PlainText<String> {
    
    match db.create_user(username, password).await {
        Ok(..) => PlainText("User created successfully".to_string()),
        Err(err) => {
            eprint!("Failed to create user: {}", err);
            PlainText("Failed to create user".to_string())
        }
    }
}
