use axum::http::StatusCode;

use crate::{
    adapters::shared::di::auth_domain,
    application::usecases::auth_usecases::{
        AuthUseCases, VerifyIdTokenRequest, VerifyIdTokenResponse,
    },
    core::{
        http::{failure::HttpFailure, response::HttpResponse},
        middlewares::validator::ValidatedPayload,
    },
};

pub async fn execute(
    ValidatedPayload(req): ValidatedPayload<VerifyIdTokenRequest>,
) -> Result<HttpResponse<VerifyIdTokenResponse>, HttpFailure> {
    let auth_response = auth_domain()
        .verify_token(req)
        .await
        .map_err(|failure| HttpFailure::new(failure))?;

    let response = HttpResponse {
        status: StatusCode::OK,
        message: "Verify token successfully !!!".to_string(),
        payload: auth_response,
    };

    Ok(response)
}
