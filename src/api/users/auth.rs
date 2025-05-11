use crate::api::BasicAuthentication;
use poem::http::StatusCode;
use poem::Error;

impl crate::Api {
    pub async fn auth(&self, auth: BasicAuthentication) -> Result<(String, String), Error> {
        if auth.0.username == "" || auth.0.password == "" {
            return Err(Error::from_status(StatusCode::UNAUTHORIZED));
        }
        self.db.validate_password(&auth.0.username, &auth.0.password)
            .await
            .map_err(|_| Error::from_status(StatusCode::UNAUTHORIZED))?;
        Ok((auth.0.username, auth.0.password))
    }
}