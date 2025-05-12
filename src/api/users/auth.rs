use poem::http::StatusCode;
use poem::{Error, FromRequest, RequestBody};
use poem_openapi::payload::PlainText;

impl crate::Api {
    pub async fn auth(&self, auth: &ApiAuthentication) -> Result<(), Error> {
        if auth.username == "" || auth.password == "" {
            return Err(Error::from_status(StatusCode::UNAUTHORIZED));
        }
        self.db.validate_password(&auth.username, &auth.password)
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
pub struct ApiAuthentication {
    pub username: String,
    pub password: String,
}

impl FromRequest<'_> for ApiAuthentication {
    async fn from_request(req: &poem::Request, body: &mut RequestBody) -> poem::Result<Self> {
        let username = req.headers().get("x-auth-user").and_then(|h| h.to_str().ok()).unwrap_or("");
        let password = req.headers().get("x-auth-key").and_then(|h| h.to_str().ok()).unwrap_or("");

        Ok(ApiAuthentication {
            username: username.to_string(),
            password: password.to_string(),
        })
    }
}
