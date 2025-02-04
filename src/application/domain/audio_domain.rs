use std::sync::Arc;

use crate::{
    application::{
        apis::audio_api::AudioApi,
        services::audio_service::AudioService,
        usecases::audio_usecases::{
            DownloadAudioRequest, DownloadAudioUseCase, ManageAudioUseCase, SearchAudioResponse,
        },
    },
    core::{base::base_query::SearchQuery, http::failure::Failure},
};

pub struct AudioDomain {
    pub audio_service: Arc<dyn AudioService>,
    pub audio_api: Arc<dyn AudioApi>,
}

impl AudioDomain {
    pub fn new(audio_service: Arc<dyn AudioService>, audio_api: Arc<dyn AudioApi>) -> Self {
        AudioDomain {
            audio_service,
            audio_api,
        }
    }
}

#[async_trait::async_trait]
impl DownloadAudioUseCase for AudioDomain {
    /// Download And Save Audio in server
    ///
    /// This function will download audio and save it in server for future use
    /// Follow steps:
    /// 1. Create new audio entity and save it in database
    /// 2. Download audio and thumbnail and update entity in database
    /// 3. Return entity
    async fn download_and_save(&self, req: &DownloadAudioRequest) -> Result<(), Failure> {
        let response = self.audio_api.download_audio(&req.url).await?;
        println!("Response: {}", response);

        Ok(())
    }
}

#[async_trait::async_trait]
impl ManageAudioUseCase for AudioDomain {
    async fn search(&self, req: &SearchQuery) -> Result<SearchAudioResponse, Failure> {
        self.audio_service.search(req).await
    }
}
