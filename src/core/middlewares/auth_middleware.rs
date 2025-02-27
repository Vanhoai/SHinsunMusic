use axum::{
    extract::{Request, State},
    http::HeaderMap,
    middleware::Next,
    response::Response,
};
use std::sync::Arc;

use crate::{
    core::{
        http::failure::{Failure, HttpFailure},
        jwt::types::TokenType,
    },
    state::AppState,
};

pub async fn execute(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, HttpFailure> {
    let authorization = headers.get("Authorization");

    if authorization.is_none() {
        return Err(HttpFailure::new(Failure::Unauthorized(
            "Please insert bearer token to authorization headers".to_string(),
        )));
    }

    let token = authorization.unwrap().to_str().ok();
    if token.is_none() {
        return Err(HttpFailure::new(Failure::Unauthorized(
            "Please insert bearer token to authorization headers".to_string(),
        )));
    }

    let parts: Vec<&str> = token.unwrap().split_whitespace().collect();
    if parts.len() != 2 {
        return Err(HttpFailure::new(Failure::Unauthorized(
            "Please insert bearer token to authorization headers".to_string(),
        )));
    }

    let token = parts[1];
    let claims = state
        .jwt_service
        .decode_token(TokenType::AccessToken, token)
        .map_err(HttpFailure::new)?;

    if claims.exp < chrono::Utc::now().timestamp() as usize {
        return Err(HttpFailure::new(Failure::Unauthorized(
            "Token expired".to_string(),
        )));
    }

    request.extensions_mut().insert(claims);
    Ok(next.run(request).await)
}
