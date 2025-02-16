use axum::{extract::Query, http::StatusCode};

use crate::{
    adapters::shared::di::audio_domain,
    application::{
        entities::audio_entity::AudioResponse, usecases::audio_usecases::ManageAudioUseCase,
    },
    core::{
        base::base_query::SearchQuery,
        http::{failure::HttpFailure, response::HttpPaginationResponse},
    },
};

pub async fn execute(
    query: Query<SearchQuery>,
) -> Result<HttpPaginationResponse<AudioResponse>, HttpFailure> {
    let response = audio_domain()
        .search(&query)
        .await
        .map_err(HttpFailure::new)?;

    let http_response = HttpPaginationResponse {
        status: StatusCode::OK,
        message: "Sign in successfully !!!".to_string(),
        meta: response.meta,
        payload: AudioResponse::array(response.audios),
    };

    Ok(http_response)
}
