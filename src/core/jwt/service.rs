use jsonwebtoken::{decode, encode, Algorithm, Header, Validation};

use super::{claims::Claims, interface::JwtService, types::TokenType};
use crate::core::{
    configs::app_config::APP_CONFIG, cryptography::keypair::KEY_PAIR, http::failure::Failure,
};

pub struct JwtServiceImpl {}

impl JwtServiceImpl {
    pub fn new() -> Self {
        JwtServiceImpl {}
    }
}

impl Default for JwtServiceImpl {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl JwtService for JwtServiceImpl {
    fn encode_token(&self, token_type: TokenType, claims: &Claims) -> Result<String, Failure> {
        let private_key = match token_type {
            TokenType::AccessToken => &KEY_PAIR.access_private_key,
            TokenType::RefreshToken => &KEY_PAIR.refresh_private_key,
        };

        let algorithms = match APP_CONFIG.jwt.key_type.as_str() {
            "Elliptic" => Algorithm::ES256,
            "RSA" => Algorithm::RS256,
            _ => Algorithm::RS256,
        };

        let encoded = encode(&Header::new(algorithms), &claims, private_key)
            .map_err(|e| Failure::InternalServerError(e.to_string()))?;

        Ok(encoded)
    }

    fn decode_token(&self, token_type: TokenType, token: &str) -> Result<Claims, Failure> {
        let public_key = match token_type {
            TokenType::AccessToken => &KEY_PAIR.access_public_key,
            TokenType::RefreshToken => &KEY_PAIR.refresh_public_key,
        };

        let algorithms = match APP_CONFIG.jwt.key_type.as_str() {
            "Elliptic" => Algorithm::ES256,
            "RSA" => Algorithm::RS256,
            _ => Algorithm::RS256,
        };

        let result = decode::<Claims>(token, public_key, &Validation::new(algorithms))
            .map_err(|e| Failure::Unauthorized(e.to_string()))?;

        Ok(result.claims)
    }
}
