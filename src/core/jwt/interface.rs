use crate::core::http::failure::Failure;

use super::{claims::Claims, types::TokenType};

#[async_trait::async_trait]
pub trait JwtService: Send + Sync {
    fn encode_token(&self, token_type: TokenType, claims: &Claims) -> Result<String, Failure>;
    fn decode_token(&self, token: &str, token_type: TokenType) -> Result<Claims, Failure>;
}
