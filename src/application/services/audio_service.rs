use async_trait::async_trait;
use std::sync::Arc;

use crate::{
    application::{
        repositories::audio_repository::AudioRepository,
        usecases::audio_usecases::SearchAudioResponse,
    },
    core::{base::base_query::SearchQuery, http::failure::Failure},
};

#[async_trait]
pub trait AudioService: Send + Sync {
    async fn search(&self, req: &SearchQuery) -> Result<SearchAudioResponse, Failure>;
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
        todo!()
    }
}
