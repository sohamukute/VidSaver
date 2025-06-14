use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct VideoUrlRequest {
    pub url: String,
}

#[derive(Debug, Serialize)]
pub struct VideoInfo {
    pub url: String,
    pub title: String,
    pub thumbnail: String,
    pub duration: String,
    pub views: String,
    pub uploader: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct VideoFormat {
    pub format_id: String,
    pub quality: String,
    pub ext: String,
    pub filesize: Option<u64>,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct AudioFormat {
    pub format_id: String,
    pub ext: String,
    pub abr: u32,
    pub filesize: Option<u64>,
}

#[derive(Debug, Serialize)]
pub struct QualityOptions {
    pub video: Vec<VideoFormat>,
    pub audio: Vec<AudioFormat>,
}

#[derive(Debug, Deserialize)]
pub struct DownloadRequest {
    pub url: String,
    pub r#type: String, // 'video', 'audio', or 'mp3'
    pub video_quality: Option<String>,
    pub audio_quality: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ApiError {
    pub error: String,
    pub message: String,
}