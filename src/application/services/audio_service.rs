use async_trait::async_trait;
use std::sync::Arc;

use crate::{
    application::{
        entities::audio_entity::AudioEntity, repositories::audio_repository::AudioRepository,
        usecases::audio_usecases::SearchAudioResponse,
    },
    core::{base::base_query::SearchQuery, http::failure::Failure},
};

#[async_trait]
pub trait AudioService: Send + Sync {
    async fn search(&self, req: &SearchQuery) -> Result<SearchAudioResponse, Failure>;
    async fn create(&self, req: &AudioEntity) -> Result<AudioEntity, Failure>;
    async fn update(&self, req: &AudioEntity) -> Result<AudioEntity, Failure>;
    async fn find_one_by_id(&self, id: &str) -> Result<AudioEntity, Failure>;
}

pub struct AudioServiceImpl {
    pub repository: Arc<dyn AudioRepository>,
}

impl AudioServiceImpl {
    pub fn new(repository: Arc<dyn AudioRepository>) -> Self {
        AudioServiceImpl { repository }
    }
}

#[async_trait]
impl AudioService for AudioServiceImpl {
    async fn search(&self, req: &SearchQuery) -> Result<SearchAudioResponse, Failure> {
        self.repository.search(req).await
    }

    async fn find_one_by_id(&self, id: &str) -> Result<AudioEntity, Failure> {
        let response = self.repository.find_by_id(id).await?;
        if response.is_none() {
            return Err(Failure::NotFound("Audio not found".to_string()));
        }

        Ok(response.unwrap())
    }

    async fn update(&self, req: &AudioEntity) -> Result<AudioEntity, Failure> {
        self.repository.update(req).await
    }

    async fn create(&self, req: &AudioEntity) -> Result<AudioEntity, Failure> {
        let option_audio = self.repository.find_by_yt_id(&req.yt_id).await?;
        if option_audio.is_some() {
            return Err(Failure::Conflict(
                "Audio with this yt_id already exists".to_string(),
            ));
        }

        self.repository.create(req).await
    }
}
