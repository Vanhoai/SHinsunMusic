use axum::{extract::Query, http::StatusCode};

use crate::{
    adapters::shared::di::audio_domain,
    application::{
        entities::audio_entity::AudioEntity, usecases::audio_usecases::ManageAudioUseCase,
    },
    core::{
        base::base_query::SearchQuery,
        http::{failure::HttpFailure, response::HttpPaginationResponse},
    },
};

pub async fn execute(
    query: Query<SearchQuery>,
) -> Result<HttpPaginationResponse<AudioEntity>, HttpFailure> {
    let response = audio_domain()
        .search(&query)
        .await
        .map_err(|failure| HttpFailure::new(failure))?;

    let http_response = HttpPaginationResponse {
        status: StatusCode::OK,
        message: "Sign in successfully !!!".to_string(),
        meta: response.meta,
        payload: response.audios,
    };

    Ok(http_response)
}
