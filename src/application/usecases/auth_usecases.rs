use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::core::http::failure::Failure;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AuthRequest {
    #[validate(length(min = 20))]
    pub uuid: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 5))]
    pub name: String,
    #[validate(length(min = 5))]
    pub avatar: String,
    #[validate(length(min = 20))]
    pub device_token: String,
    #[validate(length(min = 20))]
    pub id_token: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct VerifyIdTokenRequest {
    #[validate(length(min = 20))]
    pub id_token: String,
    #[validate(length(min = 10))]
    pub uuid: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct VerifyIdTokenResponse {
    pub exp: u64,
    pub iat: u64,
    pub iss: String,
    pub sub: String,
    pub auth_time: u64,
}

#[async_trait]
pub trait AuthUseCases: Send + Sync {
    async fn verify_token(
        &self,
        id_token: VerifyIdTokenRequest,
    ) -> Result<VerifyIdTokenResponse, Failure>;

    async fn sign_in(&self, req: AuthRequest) -> Result<AuthResponse, Failure>;
}
