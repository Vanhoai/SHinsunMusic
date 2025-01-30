use jsonwebtoken::{decode, encode, Algorithm, Header, Validation};

use super::{claims::Claims, interface::JwtService, types::TokenType};
use crate::core::{cryptography::keypair::KEY_PAIR, http::failure::Failure};

pub struct JwtServiceImpl {}

impl JwtServiceImpl {
    pub fn new() -> Self {
        JwtServiceImpl {}
    }
}

#[async_trait::async_trait]
impl JwtService for JwtServiceImpl {
    fn encode_token(&self, token_type: TokenType, claims: &Claims) -> Result<String, Failure> {
        let private_key = match token_type {
            TokenType::AccessToken => &KEY_PAIR.access_private_key,
            TokenType::RefreshToken => &KEY_PAIR.refresh_private_key,
        };

        let encoded = encode(&Header::new(Algorithm::RS256), &claims, &private_key)
            .map_err(|e| Failure::InternalServerError(e.to_string()))?;

        Ok(encoded)
    }

    fn decode_token(&self, token: &str, token_type: TokenType) -> Result<Claims, Failure> {
        let public_key = match token_type {
            TokenType::AccessToken => &KEY_PAIR.access_public_key,
            TokenType::RefreshToken => &KEY_PAIR.refresh_public_key,
        };

        let result = decode::<Claims>(&token, &public_key, &Validation::new(Algorithm::RS256))
            .map_err(|e| Failure::Unauthorized(e.to_string()))?;

        Ok(result.claims)
    }
}
