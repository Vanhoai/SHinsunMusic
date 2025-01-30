use crate::{
    application::usecases::auth_usecases::VerifyIdTokenResponse, core::http::failure::Failure,
};

#[async_trait::async_trait]
pub trait AuthApi: Send + Sync {
    async fn verify_id_token(&self, id_token: &str) -> Result<VerifyIdTokenResponse, Failure>;
}
