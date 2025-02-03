use async_trait::async_trait;
use std::sync::Arc;

use crate::application::repositories::audio_repository::AudioRepository;

#[async_trait]
pub trait AudioService: Send + Sync {}

pub struct AudioServiceImpl {
    pub repository: Arc<dyn AudioRepository>,
}

impl AudioServiceImpl {
    pub fn new(repository: Arc<dyn AudioRepository>) -> Self {
        AudioServiceImpl { repository }
    }
}

#[async_trait]
impl AudioService for AudioServiceImpl {}
