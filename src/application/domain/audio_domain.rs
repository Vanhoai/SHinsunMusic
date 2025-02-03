use std::sync::Arc;

use crate::{
    application::{
        apis::audio_api::AudioApi,
        usecases::audio_usecases::{DownloadAudioRequest, DownloadAudioUseCase},
    },
    core::http::failure::Failure,
};

pub struct AudioDomain {
    pub audio_api: Arc<dyn AudioApi>,
}

impl AudioDomain {
    pub fn new(audio_api: Arc<dyn AudioApi>) -> Self {
        AudioDomain { audio_api }
    }
}

#[async_trait::async_trait]
impl DownloadAudioUseCase for AudioDomain {
    async fn download_audio(&self, req: &DownloadAudioRequest) -> Result<(), Failure> {
        let response = self.audio_api.download_audio(&req.url).await?;
        println!("Response: {}", response);

        Ok(())
    }
}
