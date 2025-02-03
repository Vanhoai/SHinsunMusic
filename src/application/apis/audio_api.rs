use crate::core::http::failure::Failure;

#[async_trait::async_trait]
pub trait AudioApi: Send + Sync {
    async fn download_audio(&self, url: &str) -> Result<String, Failure>;
    async fn download_thumbnail(&self, url: &str) -> Result<String, Failure>;
}
