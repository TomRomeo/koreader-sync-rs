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
        let hashed_password = match self.db.get_hashed_password(&username.0.key).await {
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
        Ok(())
    }
}

pub async fn handler(db: & dyn crate::db::Database, username: &str, password: &str) -> Result<PlainText<String>, Error> {
    
    let hashed_password = match db.get_hashed_password(username).await {
        Ok(hash) => hash,
        Err(err) => {
            eprintln!("Failed to get hashed password: {}", err);
            return Err(Error::from_status(StatusCode::UNAUTHORIZED));
        }
    };
    
    if !bcrypt::verify(password, &hashed_password).unwrap_or(false) {
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
