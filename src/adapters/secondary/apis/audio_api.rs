use std::path::PathBuf;
use yt_dlp::fetcher::deps::Libraries;
use yt_dlp::Youtube;

use crate::{
    application::apis::audio_api::{
        AudioApi, DownloadAudioApiRequest, DownloadAudioResponse, DownloadThumbnailResponse,
    },
    core::http::failure::Failure,
};

pub struct AudioApiImpl {}

impl AudioApiImpl {
    pub fn new() -> Self {
        AudioApiImpl {}
    }
}

impl Default for AudioApiImpl {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl AudioApi for AudioApiImpl {
    async fn download_audio(
        &self,
        req: &DownloadAudioApiRequest,
    ) -> Result<DownloadAudioResponse, Failure> {
        let libraries_dir = PathBuf::from("libraries");
        let output_dir = PathBuf::from("outputs");

        let youtube = libraries_dir.join("yt-dlp");
        let ffmpeg = libraries_dir.join("ffmpeg");

        let libraries = Libraries::new(youtube, ffmpeg);
        println!("load libraries successful");
        println!("fetch libraries and outputs dir");
        let fetcher =
            Youtube::new(libraries, output_dir).map_err(|e| Failure::BadRequest(e.to_string()))?;

        let url = req.url.clone();
        let video = fetcher
            .fetch_video_infos(url)
            .await
            .map_err(|e| Failure::BadRequest(e.to_string()))?;

        println!("Video title: {}", video.title);

        let audio_format = video.best_audio_format().unwrap();
        let audio_path = fetcher
            .download_format(audio_format, "audio.mp3")
            .await
            .map_err(|e| Failure::BadRequest(e.to_string()))?;

        println!("download audio successful");

        Ok(DownloadAudioResponse {
            audio: audio_path.display().to_string(),
            name: video.title.clone(),
        })
    }

    async fn download_thumbnail(
        &self,
        req: &DownloadAudioApiRequest,
    ) -> Result<DownloadThumbnailResponse, Failure> {
        let libraries_dir = PathBuf::from("libraries");
        let output_dir = PathBuf::from(req.location.as_str());

        let youtube = libraries_dir.join("yt-dlp");
        let ffmpeg = libraries_dir.join("ffmpeg");

        let libraries = Libraries::new(youtube, ffmpeg);
        let fetcher =
            Youtube::new(libraries, output_dir).map_err(|e| Failure::BadRequest(e.to_string()))?;

        println!("load libraries thumbnail");

        let url = req.url.clone();
        let file_name = format!("youtube_{}_thumbnail.jpg", req.yt_id);
        println!("file name thumbnail: {:?}", file_name);
        let thumbnail_path = fetcher
            .download_thumbnail_from_url(url, file_name.as_str())
            .await
            .map_err(|e| Failure::BadRequest(e.to_string()))?;

        Ok(DownloadThumbnailResponse {
            thumbnail: thumbnail_path.display().to_string(),
        })
    }
}
