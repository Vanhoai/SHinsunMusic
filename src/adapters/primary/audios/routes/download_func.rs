use axum::http::StatusCode;

use crate::{
    adapters::shared::di::audio_domain,
    application::usecases::audio_usecases::{DownloadAudioRequest, DownloadAudioUseCase},
    core::{
        http::{failure::HttpFailure, response::HttpResponse},
        middlewares::validator_middleware::ValidatedMiddleware,
    },
};

pub async fn execute(
    ValidatedMiddleware(req): ValidatedMiddleware<DownloadAudioRequest>,
) -> Result<HttpResponse<String>, HttpFailure> {
    audio_domain()
        .download_and_save(&req)
        .await
        .map_err(HttpFailure::new)?;

    let http_response = HttpResponse {
        status: StatusCode::OK,
        message: "Sign in successfully !!!".to_string(),
        payload: "success".to_string(),
    };

    Ok(http_response)
}
