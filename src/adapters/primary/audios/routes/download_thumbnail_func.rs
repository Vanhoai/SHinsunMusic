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
    let response = audio_domain()
        .download_thumbnail(&req)
        .await
        .map_err(|failure| HttpFailure::new(failure))?;

    let http_response = HttpResponse {
        status: StatusCode::OK,
        message: "Download thumbnail successfully !!!".to_string(),
        payload: response,
    };

    Ok(http_response)
}
