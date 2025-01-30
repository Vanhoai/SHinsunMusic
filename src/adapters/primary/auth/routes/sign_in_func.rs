use axum::http::StatusCode;

use crate::{
    adapters::shared::di::auth_domain,
    application::usecases::auth_usecases::{AuthRequest, AuthResponse, AuthUseCases},
    core::{
        http::{failure::HttpFailure, response::HttpResponse},
        middlewares::validator::ValidatedPayload,
    },
};

pub async fn execute(
    ValidatedPayload(req): ValidatedPayload<AuthRequest>,
) -> Result<HttpResponse<AuthResponse>, HttpFailure> {
    let response = auth_domain()
        .sign_in(req)
        .await
        .map_err(|failure| HttpFailure::new(failure))?;

    let http_response = HttpResponse {
        status: StatusCode::OK,
        message: "Sign in successfully !!!".to_string(),
        payload: response,
    };

    Ok(http_response)
}
