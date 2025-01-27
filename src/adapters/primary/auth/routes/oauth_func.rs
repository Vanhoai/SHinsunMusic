use axum::http::StatusCode;

use crate::{
    adapters::shared::di::auth_domain,
    application::usecases::auth_usecases::AuthUseCases,
    application::usecases::auth_usecases::{AuthResponse, OAuthRequest},
    core::{
        http::{failure::HttpFailure, response::HttpResponse},
        middlewares::validator::ValidatedPayload,
    },
};

pub async fn execute(
    ValidatedPayload(req): ValidatedPayload<OAuthRequest>,
) -> Result<HttpResponse<AuthResponse>, HttpFailure> {
    let auth_response = auth_domain()
        .oauth(req)
        .await
        .map_err(|failure| HttpFailure::new(failure))?;

    let response = HttpResponse {
        status: StatusCode::OK,
        message: "Sign in successfully !!!".to_string(),
        payload: auth_response,
    };

    Ok(response)
}
