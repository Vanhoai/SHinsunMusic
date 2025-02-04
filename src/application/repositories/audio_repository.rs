use async_trait::async_trait;

use crate::application::entities::audio_entity::AudioEntity;
use crate::application::usecases::audio_usecases::SearchAudioResponse;
use crate::core::base::base_query::SearchQuery;
use crate::core::http::failure::Failure;

#[async_trait]
pub trait AudioRepository: Send + Sync {
    async fn create(&self, data: &AudioEntity) -> Result<AudioEntity, Failure>;
    async fn update(&self, data: &AudioEntity) -> Result<AudioEntity, Failure>;
    async fn find_by_id(&self, id: &str) -> Result<Option<AudioEntity>, Failure>;
    async fn find_by_yt_id(&self, yt_id: &str) -> Result<Option<AudioEntity>, Failure>;
    async fn search(&self, query: &SearchQuery) -> Result<SearchAudioResponse, Failure>;
}
