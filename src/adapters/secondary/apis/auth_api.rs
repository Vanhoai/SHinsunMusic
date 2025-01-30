use crate::{
    adapters::shared::firebase::firebase_app::firebase,
    application::{apis::auth_api::AuthApi, usecases::auth_usecases::VerifyIdTokenResponse},
    core::http::failure::Failure,
};

pub struct AuthApiImpl {}

impl AuthApiImpl {
    pub fn new() -> Self {
        AuthApiImpl {}
    }
}

#[async_trait::async_trait]
impl AuthApi for AuthApiImpl {
    async fn verify_id_token(&self, id_token: &str) -> Result<VerifyIdTokenResponse, Failure> {
        let response = firebase().verify_auth_token(id_token).await?;

        Ok(VerifyIdTokenResponse {
            exp: response.exp,
            iat: response.iat,
            iss: response.iss,
            sub: response.sub,
            auth_time: response.auth_time,
        })
    }
}
