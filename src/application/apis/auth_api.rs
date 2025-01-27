use crate::core::http::failure::Failure;

pub struct VerifyIdTokenResponse {}

#[async_trait::async_trait]
pub trait AuthApi: Send + Sync {
    async fn verify_id_token(&self, id_token: String) -> Result<VerifyIdTokenResponse, Failure>;
}
