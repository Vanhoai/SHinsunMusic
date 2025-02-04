use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    application::entities::audio_entity::AudioEntity,
    core::{
        base::base_query::SearchQuery,
        http::{failure::Failure, response::Meta},
    },
};

/// ====================== For Download Audio Use Case ======================
#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DownloadAudioRequest {
    #[validate(url)]
    pub url: String,
}

#[async_trait::async_trait]
pub trait DownloadAudioUseCase: Send + Sync {
    async fn download_and_save(&self, req: &DownloadAudioRequest) -> Result<(), Failure>;
}

/// ====================== For Manage Audio Use Case ======================
#[derive(Debug, Serialize, Deserialize)]
pub struct SearchAudioResponse {
    pub meta: Meta,
    pub audios: Vec<AudioEntity>,
}

#[async_trait::async_trait]
pub trait ManageAudioUseCase: Send + Sync {
    async fn search(&self, req: &SearchQuery) -> Result<SearchAudioResponse, Failure>;
}
