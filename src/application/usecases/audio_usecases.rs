use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    application::entities::audio_entity::{AudioEntity, AudioResponse},
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
    async fn download_audio(&self, req: &DownloadAudioRequest) -> Result<String, Failure>;
    async fn download_thumbnail(&self, req: &DownloadAudioRequest) -> Result<String, Failure>;
}

/// ====================== For Manage Audio Use Case ======================
#[derive(Debug, Serialize, Deserialize)]
pub struct SearchAudioResponse {
    pub meta: Meta,
    pub audios: Vec<AudioEntity>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateWithExistFileRequest {
    pub name: String,
    pub yt_id: String,
}

#[async_trait::async_trait]
pub trait ManageAudioUseCase: Send + Sync {
    async fn search(&self, req: &SearchQuery) -> Result<SearchAudioResponse, Failure>;
    async fn create_with_exist_file(
        &self,
        req: &CreateWithExistFileRequest,
    ) -> Result<AudioEntity, Failure>;

    async fn find_audio(&self, id: &str) -> Result<AudioResponse, Failure>;
}
