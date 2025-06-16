use axum::{
    extract::Json,
    http::{header, StatusCode},
    response::{IntoResponse, Response},
};
use crate::services::youtube_service;
use crate::types::*;

pub async fn get_video_info(Json(request): Json<VideoInfoRequest>) -> Result<Json<VideoInfo>, AppError> {
    let video_info = youtube_service::extract_video_info(&request.url).await?;
    Ok(Json(video_info))
}

pub async fn get_quality_options(Json(request): Json<QualityOptionsRequest>) -> Result<Json<QualityOptions>, AppError> {
    let quality_options = youtube_service::extract_quality_options(&request.url).await?;
    Ok(Json(quality_options))
}

pub async fn download_video(Json(request): Json<DownloadRequest>) -> Result<Response, AppError> {
    let (file_data, filename) = youtube_service::download_video(request).await?;
    
    let content_type = match filename.split('.').last() {
        Some("mp4") => "video/mp4",
        Some("webm") => "video/webm",
        Some("mp3") => "audio/mpeg",
        Some("m4a") => "audio/mp4",
        _ => "application/octet-stream",
    };

    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, content_type)
        .header(
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{}\"", filename),
        )
        .header(header::CONTENT_LENGTH, file_data.len())
        .body(axum::body::Body::from(file_data))
        .unwrap();

    Ok(response)
}

// Error handling
#[derive(Debug)]
pub struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}