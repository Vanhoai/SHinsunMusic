use axum::{extract::Path, http::StatusCode};

use crate::{
    adapters::shared::di::audio_domain,
    application::{
        entities::audio_entity::AudioResponse, usecases::audio_usecases::ManageAudioUseCase,
    },
    core::http::{failure::HttpFailure, response::HttpResponse},
};

pub async fn execute(Path(id): Path<String>) -> Result<HttpResponse<AudioResponse>, HttpFailure> {
    let response = audio_domain()
        .find_audio(&id)
        .await
        .map_err(HttpFailure::new)?;

    let http_response = HttpResponse {
        status: StatusCode::OK,
        message: "Find audio successfully !!!".to_string(),
        payload: response,
    };

    Ok(http_response)
}
