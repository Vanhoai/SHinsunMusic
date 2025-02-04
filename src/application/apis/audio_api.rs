use crate::core::http::failure::Failure;

pub struct DownloadAudioApiRequest {
    pub yt_id: String,
    pub url: String,
    pub location: String,
}

#[derive(Debug)]
pub struct DownloadAudioResponse {
    pub audio: String,
    pub name: String,
}

#[derive(Debug)]
pub struct DownloadThumbnailResponse {
    pub thumbnail: String,
}

#[async_trait::async_trait]
pub trait AudioApi: Send + Sync {
    async fn download_audio(
        &self,
        req: &DownloadAudioApiRequest,
    ) -> Result<DownloadAudioResponse, Failure>;

    async fn download_thumbnail(
        &self,
        req: &DownloadAudioApiRequest,
    ) -> Result<DownloadThumbnailResponse, Failure>;
}
