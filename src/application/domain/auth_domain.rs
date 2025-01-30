use async_trait::async_trait;
use std::sync::Arc;

use crate::{
    application::{
        apis::auth_api::AuthApi,
        services::auth_service::AuthService,
        usecases::auth_usecases::{
            AuthRequest, AuthResponse, AuthUseCases, VerifyIdTokenRequest, VerifyIdTokenResponse,
        },
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
    async fn sign_in(&self, _: AuthRequest) -> Result<AuthResponse, Failure> {
        todo!()
    }

    async fn verify_token(
        &self,
        req: VerifyIdTokenRequest,
    ) -> Result<VerifyIdTokenResponse, Failure> {
        let response = self.auth_api.verify_id_token(&req.id_token).await?;

        if response.sub != req.uuid {
            return Err(Failure::Unauthorized(
                "Invalid account with uuid".to_string(),
            ));
        }

        Ok(response)
    }
}
