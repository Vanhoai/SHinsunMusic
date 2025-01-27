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
pub struct OAuthRequest {
    #[validate(length(min = 20))]
    pub id_token: String,
    #[validate(length(min = 20))]
    pub device_token: String,
}

#[async_trait]
pub trait AuthUseCases: Send + Sync {
    async fn oauth(&self, req: OAuthRequest) -> Result<AuthResponse, Failure>;
}
