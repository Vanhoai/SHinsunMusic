use async_trait::async_trait;

use crate::application::entities::audio_entity::AudioEntity;
use crate::core::http::failure::Failure;

#[async_trait]
pub trait AudioRepository: Send + Sync {
    async fn create(&self, data: &AudioEntity) -> Result<AudioEntity, Failure>;
    async fn find_by_id(&self, id: &str) -> Result<Option<AudioEntity>, Failure>;
}
