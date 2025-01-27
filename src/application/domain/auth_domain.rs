use std::sync::Arc;

use crate::{
    application::{
        apis::auth_api::AuthApi,
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
            auth_service: auth_service,
            auth_api: auth_api,
        }
    }
}

#[async_trait::async_trait]
impl AuthUseCases for AuthDomain {
    async fn oauth(&self, _: OAuthRequest) -> Result<AuthResponse, Failure> {
        todo!()
    }
}
