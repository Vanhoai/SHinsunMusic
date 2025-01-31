use axum::http::StatusCode;

use crate::{
    adapters::shared::di::auth_domain,
    application::usecases::auth_usecases::{AuthRequest, AuthResponse, AuthUseCases},
    core::{
        http::{failure::HttpFailure, response::HttpResponse},
        middlewares::validator_middleware::ValidatedMiddleware,
    },
};

pub async fn execute(
    ValidatedMiddleware(req): ValidatedMiddleware<AuthRequest>,
) -> Result<HttpResponse<AuthResponse>, HttpFailure> {
    println!("Request: {:?}", req);

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
