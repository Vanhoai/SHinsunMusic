use axum::http::StatusCode;

use crate::{
    adapters::shared::di::audio_domain,
    application::{
        entities::audio_entity::AudioEntity,
        usecases::audio_usecases::{CreateWithExistFileRequest, ManageAudioUseCase},
    },
    core::{
        http::{failure::HttpFailure, response::HttpResponse},
        middlewares::validator_middleware::ValidatedMiddleware,
    },
};

pub async fn execute(
    ValidatedMiddleware(req): ValidatedMiddleware<CreateWithExistFileRequest>,
) -> Result<HttpResponse<AudioEntity>, HttpFailure> {
    let response = audio_domain()
        .create_with_exist_file(&req)
        .await
        .map_err(|failure| HttpFailure::new(failure))?;

    let http_response = HttpResponse {
        status: StatusCode::OK,
        message: "Create audio successfully !!!".to_string(),
        payload: response,
    };

    Ok(http_response)
}
