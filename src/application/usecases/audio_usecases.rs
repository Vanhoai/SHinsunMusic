use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::core::http::failure::Failure;

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DownloadAudioRequest {
    #[validate(url)]
    pub url: String,
}

#[async_trait::async_trait]
pub trait DownloadAudioUseCase: Send + Sync {
    async fn download_audio(&self, req: &DownloadAudioRequest) -> Result<(), Failure>;
}
