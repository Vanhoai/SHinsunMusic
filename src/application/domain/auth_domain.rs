use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};
use std::sync::Arc;

use crate::{
    application::{
        apis::auth_api::AuthApi,
        entities::account_entity::AccountEntity,
        services::{account_service::AccountService, auth_service::AuthService},
        usecases::auth_usecases::{
            AuthRequest, AuthResponse, AuthUseCases, VerifyIdTokenRequest, VerifyIdTokenResponse,
        },
    },
    core::http::failure::Failure,
};

pub struct AuthDomain {
    pub auth_service: Arc<dyn AuthService>,
    pub account_service: Arc<dyn AccountService>,
    pub auth_api: Arc<dyn AuthApi>,
}

impl AuthDomain {
    pub fn new(
        auth_service: Arc<dyn AuthService>,
        account_service: Arc<dyn AccountService>,
        auth_api: Arc<dyn AuthApi>,
    ) -> Self {
        AuthDomain {
            auth_service,
            account_service,
            auth_api,
        }
    }
}

#[async_trait::async_trait]
impl AuthUseCases for AuthDomain {
    async fn sign_in(&self, req: AuthRequest) -> Result<AuthResponse, Failure> {
        self.auth_api.verify_id_token(&req.id_token).await?;

        let account = self.account_service.find_by_email(&req.email).await;
        if account.is_err() {
            let data = AccountEntity::new(
                req.uuid,
                req.name,
                req.email,
                "".to_string(),
                req.avatar,
                req.device_token,
            );
            let new_account = self.account_service.create_account(&data);
        }

        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password("".to_string().as_bytes(), &salt)
            .unwrap();

        Err(Failure::BadRequest("Invalid email".to_string()))
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
