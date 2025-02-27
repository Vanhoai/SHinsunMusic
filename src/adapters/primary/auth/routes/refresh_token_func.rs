use axum::http::StatusCode;

use crate::{
    adapters::shared::di::auth_domain,
    application::usecases::auth_usecases::{AuthResponse, AuthUseCases, RefreshTokenReq},
    core::{
        http::{failure::HttpFailure, response::HttpResponse},
        middlewares::validator_middleware::ValidatedMiddleware,
    },
};

pub async fn execute(
    ValidatedMiddleware(req): ValidatedMiddleware<RefreshTokenReq>,
) -> Result<HttpResponse<AuthResponse>, HttpFailure> {
    let response = auth_domain()
        .refresh_token(&req)
        .await
        .map_err(HttpFailure::new)?;

    let http_response = HttpResponse {
        status: StatusCode::OK,
        message: "Refresh Token Successfully !!!".to_string(),
        payload: response,
    };

    Ok(http_response)
}
