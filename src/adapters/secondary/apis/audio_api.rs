use std::path::PathBuf;
use yt_dlp::fetcher::deps::Libraries;
use yt_dlp::Youtube;

use crate::{application::apis::audio_api::AudioApi, core::http::failure::Failure};

pub struct AudioApiImpl {}

impl AudioApiImpl {
    pub fn new() -> Self {
        AudioApiImpl {}
    }
}

#[async_trait::async_trait]
impl AudioApi for AudioApiImpl {
    async fn download_audio(&self, url: &str) -> Result<String, Failure> {
        let libraries_dir = PathBuf::from("libraries");
        let output_dir = PathBuf::from("outputs");

        let youtube = libraries_dir.join("yt-dlp");
        let ffmpeg = libraries_dir.join("ffmpeg");

        let libraries = Libraries::new(youtube, ffmpeg);
        println!("load libraries successful");
        println!("fetch libraries and outputs dir");
        let fetcher =
            Youtube::new(libraries, output_dir).map_err(|e| Failure::BadRequest(e.to_string()))?;

        let url = String::from(url);
        let video = fetcher
            .fetch_video_infos(url)
            .await
            .map_err(|e| Failure::BadRequest(e.to_string()))?;

        println!("Video title: {}", video.title);

        let audio_format = video.worst_audio_format().unwrap();
        println!("load audio format");
        let audio_path = fetcher
            .download_format(&audio_format, "audio.mp3")
            .await
            .map_err(|e| Failure::BadRequest(e.to_string()))?;

        println!("download audio successful");
        Ok(audio_path.display().to_string())
    }

    async fn download_thumbnail(&self, url: &str) -> Result<String, Failure> {
        let libraries_dir = PathBuf::from("libs");
        let output_dir = PathBuf::from("outputs");

        let youtube = libraries_dir.join("yt-dlp");
        let ffmpeg = libraries_dir.join("ffmpeg");

        let libraries = Libraries::new(youtube, ffmpeg);
        let fetcher =
            Youtube::new(libraries, output_dir).map_err(|e| Failure::BadRequest(e.to_string()))?;

        let url = String::from(url);
        let thumbnail_path = fetcher
            .download_thumbnail_from_url(url, "thumbnail.jpg")
            .await
            .map_err(|e| Failure::BadRequest(e.to_string()))?;

        Ok(thumbnail_path.display().to_string())
    }
}
