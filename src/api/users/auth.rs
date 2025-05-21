use std::sync::Arc;
use poem::http::StatusCode;
use poem::Error;
use poem_openapi::auth::ApiKey;
use poem_openapi::payload::PlainText;
use poem_openapi::{OpenApi, SecurityScheme};
use crate::api::users;
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
    #[oai(path = "/users/auth", method = "get")]
    async fn auth_user(&self, username: AuthUser, password: AuthPassword) -> poem::Result<PlainText<String>> {
        println!("auth endpoint hit");
        
        authenticate(&*self.db, &username, &password).await
    }
}

pub async fn authenticate(
    db: &dyn Database,
    username: &AuthUser,
    password: &AuthPassword,
) -> Result<PlainText<String>, Error> {
    let hashed_password = match db.get_hashed_password(&username.0.key).await {
        Ok(hash) => hash,
        Err(err) => {
            eprintln!("Failed to get hashed password: {}", err);
            return Err(Error::from_status(StatusCode::UNAUTHORIZED));
        }
    };

    if !bcrypt::verify(&password.0.key, &hashed_password).unwrap_or(false) {
        eprintln!("Invalid password");
        return Err(Error::from_status(StatusCode::UNAUTHORIZED));
    }
    Ok(PlainText("User authenticated successfully".to_string()))
}

#[derive(SecurityScheme)]
#[oai(key_name = "x-auth-user", ty = "api_key", key_in = "header")]
pub struct AuthUser(pub ApiKey);

#[derive(SecurityScheme)]
#[oai(key_name = "x-auth-key", ty = "api_key", key_in = "header")]
pub struct AuthPassword(pub ApiKey);
