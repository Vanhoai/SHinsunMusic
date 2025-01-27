use async_trait::async_trait;
use std::sync::Arc;

use crate::{
    application::{
        apis::auth_api::{AuthApi, VerifyIdTokenResponse},
        services::auth_service::AuthService,
        usecases::auth_usecases::{AuthResponse, AuthUseCases, OAuthRequest},
    },
    core::http::failure::Failure,
};

pub struct AuthDomain {
    pub auth_service: Arc<dyn AuthService>,
    pub auth_api: Arc<dyn AuthApi>,
}

impl AuthDomain {
    pub fn new(auth_service: Arc<dyn AuthService>, auth_api: Arc<dyn AuthApi>) -> Self {
        AuthDomain {
            auth_service,
            auth_api,
        }
    }
}

#[async_trait]
impl AuthUseCases for AuthDomain {
    /// OAuth
    /// This function is used to authenticate the user using the OAuth flow.
    /// It takes an OAuthRequest as input and returns an AuthResponse.
    ///
    /// Following are the steps to authenticate the user using the OAuth flow:
    /// 1. Verify the id_token
    /// 2. Get information about the user from the id_token and firebase authentication
    /// 3. Create or get user from database
    /// 4. Create a new access, refresh token
    /// 5. Return the access and refresh token
    async fn oauth(&self, req: OAuthRequest) -> Result<AuthResponse, Failure> {
        let verify_response: VerifyIdTokenResponse =
            self.auth_api.verify_id_token(req.id_token).await?;

        Ok(AuthResponse {
            access_token: verify_response.email.clone(),
            refresh_token: verify_response.email,
        })
    }
}
