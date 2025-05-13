use poem::http::StatusCode;
use poem::Error;
use poem_openapi::auth::ApiKey;
use poem_openapi::payload::PlainText;
use poem_openapi::SecurityScheme;

impl crate::Api {
    pub async fn auth(&self, username: &AuthUser, password: &AuthPassword) -> Result<(), Error> {
        if username.0.key == "" || password.0.key == "" {
            return Err(Error::from_status(StatusCode::UNAUTHORIZED));
        }
        self.db.validate_password(&username.0.key, &password.0.key)
            .await
            .map_err(|_| Error::from_status(StatusCode::UNAUTHORIZED))?;
        Ok(())
    }
}

pub async fn handler(db: & dyn crate::db::Database, username: &str, password: &str) -> Result<PlainText<String>, Error> {

    match db.validate_password(username, password).await {
        Ok(..) => {
            Ok(PlainText("User authenticated successfully".to_string()))
        }
        Err(err) => {
            eprintln!("Failed to log in user: {}", err);
            Err(Error::from_status(StatusCode::UNAUTHORIZED))
        }
    }
}

#[derive(SecurityScheme)]
#[oai(key_name = "x-auth-user", ty = "api_key", key_in = "header")]
pub struct AuthUser(pub ApiKey);

#[derive(SecurityScheme)]
#[oai(key_name = "x-auth-key", ty = "api_key", key_in = "header")]
pub struct AuthPassword(pub ApiKey);
