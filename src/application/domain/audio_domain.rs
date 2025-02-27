use std::{path::Path, sync::Arc};

use crate::{
    application::{
        apis::audio_api::{AudioApi, DownloadAudioApiRequest},
        entities::audio_entity::{AudioEntity, AudioResponse},
        services::audio_service::AudioService,
        usecases::audio_usecases::{
            CreateWithExistFileRequest, DownloadAudioRequest, DownloadAudioUseCase,
            ManageAudioUseCase, SearchAudioResponse,
        },
    },
    core::{base::base_query::SearchQuery, helpers::string_helper, http::failure::Failure},
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
    /// FIXME: Bug here
    /// Download And Save Audio in server
    ///
    /// This function will download audio and save it in server for future use
    /// Follow steps:
    /// 1. Create new audio entity and save it in database
    /// 2. Download audio and thumbnail and update entity in database
    /// 3. Return entity
    async fn download_and_save(&self, req: &DownloadAudioRequest) -> Result<(), Failure> {
        let url = req.url.clone();
        let yt_id = string_helper::extract_youtube_id(&url);

        println!("exact yt_id: {:?}", yt_id);
        if yt_id.is_none() {
            return Err(Failure::BadRequest(
                "Can not extract yt_id from url".to_string(),
            ));
        }

        println!("create audio entity");
        let entity = AudioEntity::new(yt_id.unwrap());

        let audio = self.audio_service.create(&entity).await?;
        println!("create audio success: {:?}", audio);

        let download_audio_req = DownloadAudioApiRequest {
            yt_id: audio.yt_id.clone(),
            url: url.clone(),
            location: "outputs".to_string(),
        };

        println!("download audio and thumbnail");
        let (res_download_audio, res_download_thumbnail) = tokio::join!(
            self.audio_api.download_audio(&download_audio_req),
            self.audio_api.download_thumbnail(&download_audio_req)
        );

        let res_download_audio = res_download_audio?;
        let res_download_thumbnail = res_download_thumbnail?;

        println!("Response audio: {:?}", res_download_audio);
        println!("Response thumbnail: {:?}", res_download_thumbnail);

        let req_update = AudioEntity {
            base: audio.base,
            yt_id: audio.yt_id,
            name: res_download_audio.name,
            audio: res_download_audio.audio,
            thumbnail: res_download_thumbnail.thumbnail,
        };

        let response = self.audio_service.update(&req_update).await?;
        println!("Response update: {:?}", response);

        Ok(())
    }

    async fn download_audio(&self, _: &DownloadAudioRequest) -> Result<String, Failure> {
        todo!()
    }

    async fn download_thumbnail(&self, req: &DownloadAudioRequest) -> Result<String, Failure> {
        let url = req.url.clone();
        let yt_id = string_helper::extract_youtube_id(&url);

        println!("exact yt_id: {:?}", yt_id);
        if yt_id.is_none() {
            return Err(Failure::BadRequest(
                "Can not extract yt_id from url".to_string(),
            ));
        }

        let download_thumbnail_req = DownloadAudioApiRequest {
            yt_id: yt_id.unwrap(),
            url: url.clone(),
            location: "outputs/thumbnails".to_string(),
        };
        println!("download thumbnail");

        let response = self
            .audio_api
            .download_thumbnail(&download_thumbnail_req)
            .await?;

        Ok(response.thumbnail)
    }
}

#[async_trait::async_trait]
impl ManageAudioUseCase for AudioDomain {
    async fn search(&self, req: &SearchQuery) -> Result<SearchAudioResponse, Failure> {
        self.audio_service.search(req).await
    }

    async fn find_audio(&self, id: &str) -> Result<AudioResponse, Failure> {
        let audio = self.audio_service.find_one_by_id(id).await?;
        Ok(AudioResponse::from(audio))
    }

    async fn create_with_exist_file(
        &self,
        req: &CreateWithExistFileRequest,
    ) -> Result<AudioEntity, Failure> {
        let audio_path = format!("outputs/audios/youtube_{}_audio.mp3", req.yt_id);
        let thumbnail_path = format!("outputs/thumbnails/youtube_{}_thumbnail.jpg", req.yt_id);

        let audio_file = Path::new(&audio_path);
        let thumbnail_file = Path::new(&thumbnail_path);

        if !audio_file.exists() {
            return Err(Failure::BadRequest("Audio file not found".to_string()));
        }

        if !thumbnail_file.exists() {
            return Err(Failure::BadRequest("Thumbnail file not found".to_string()));
        }

        let entity = AudioEntity::new_full(
            req.name.clone(),
            req.yt_id.clone(),
            audio_path,
            thumbnail_path,
        );

        self.audio_service.create(&entity).await
    }
}
