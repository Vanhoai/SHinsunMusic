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
            AuthRequest, AuthResponse, AuthUseCases, RefreshTokenReq, VerifyIdTokenRequest,
            VerifyIdTokenResponse,
        },
    },
    core::{
        helpers,
        http::failure::Failure,
        jwt::{claims::Claims, interface::JwtService, types::TokenType},
    },
};

pub struct AuthDomain {
    pub auth_service: Arc<dyn AuthService>,
    pub account_service: Arc<dyn AccountService>,
    pub auth_api: Arc<dyn AuthApi>,
    pub jwt_service: Arc<dyn JwtService>,
}

impl AuthDomain {
    pub fn new(
        auth_service: Arc<dyn AuthService>,
        account_service: Arc<dyn AccountService>,
        auth_api: Arc<dyn AuthApi>,
        jwt_service: Arc<dyn JwtService>,
    ) -> Self {
        AuthDomain {
            auth_service,
            account_service,
            auth_api,
            jwt_service,
        }
    }
}

#[async_trait::async_trait]
impl AuthUseCases for AuthDomain {
    async fn sign_in(&self, req: AuthRequest) -> Result<AuthResponse, Failure> {
        let verify_response = self.auth_api.verify_id_token(&req.id_token).await?;
        if verify_response.sub != req.uuid {
            return Err(Failure::Unauthorized(
                "Invalid account with uuid".to_string(),
            ));
        }

        let mut option_account = self.account_service.find_by_email(&req.email).await?;
        let password_generated = helpers::password_generate::execute(&req.uuid, &req.email);
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        // Find account with email, if none then create new an account
        if option_account.is_none() {
            let password_hash = argon2
                .hash_password(password_generated.as_bytes(), &salt)
                .unwrap()
                .to_string();

            let data = AccountEntity::new(
                req.uuid,
                req.name,
                req.email,
                password_hash,
                req.avatar,
                req.device_token.clone(),
            );

            let new_account = self.account_service.create_account(&data).await?;
            option_account = Some(new_account);
        }

        let account = option_account.unwrap();
        let parsed_hash =
            PasswordHash::new(&account.password).map_err(|e| Failure::BadRequest(e.to_string()))?;

        argon2
            .verify_password(password_generated.as_bytes(), &parsed_hash)
            .map_err(|e| Failure::BadRequest(e.to_string()))?;

        let timestamp = jsonwebtoken::get_current_timestamp();

        // 1 hour: 60 * 60 for access token
        let mut claims = Claims {
            id: account.base.id.unwrap().to_string(),
            email: account.email.clone(),
            exp: (timestamp + 60 * 60) as usize,
            iat: timestamp as usize,
            device_token: req.device_token,
        };

        let access_token = self
            .jwt_service
            .encode_token(TokenType::AccessToken, &claims)?;

        // update expiry time 1 year: 365 * 24 * 60 * 60
        claims.exp = (timestamp + 365 * 24 * 60 * 60) as usize;

        let refresh_token = self
            .jwt_service
            .encode_token(TokenType::RefreshToken, &claims)?;

        let response = AuthResponse {
            access_token,
            refresh_token,
        };

        println!("response: {:#?}", response);
        Ok(response)
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

    /**
     * Refresh Token
     * 1. Decode refresh token
     * 2. Check expire time of refresh token
     * 3. Generate new key pair and remove refresh token in redis
     */
    async fn refresh_token(&self, req: &RefreshTokenReq) -> Result<AuthResponse, Failure> {
        let claims = self
            .jwt_service
            .decode_token(TokenType::RefreshToken, &req.refresh_token)?;

        if claims.exp < chrono::Utc::now().timestamp() as usize {
            return Err(Failure::Unauthorized("Token expired".to_string()));
        }

        let timestamp = jsonwebtoken::get_current_timestamp();
        // 1 hour: 60 * 60 for access token
        let mut new_claims = claims;
        new_claims.device_token = req.device_token.clone();
        new_claims.exp = (timestamp + 60 * 60) as usize;
        new_claims.iat = timestamp as usize;

        let access_token = self
            .jwt_service
            .encode_token(TokenType::AccessToken, &new_claims)?;

        // update expiry time 1 year: 365 * 24 * 60 * 60
        new_claims.exp = (timestamp + 365 * 24 * 60 * 60) as usize;

        let refresh_token = self
            .jwt_service
            .encode_token(TokenType::RefreshToken, &new_claims)?;

        let response = AuthResponse {
            access_token,
            refresh_token,
        };

        Ok(response)
    }
}
