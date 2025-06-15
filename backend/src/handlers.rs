use axum::{
    extract::Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};

use crate::services::youtube_service;
use crate::types::*;

#[derive(Deserialize)]
pub struct VideoInfoRequest {
    pub url: String,
}

#[derive(Deserialize)]
pub struct QualityOptionsRequest {
    pub url: String,
}

pub async fn get_video_info(
    Json(request): Json<VideoInfoRequest>,
) -> Result<Json<VideoInfo>, AppError> {
    let video_info = youtube_service::extract_video_info(&request.url).await?;
    Ok(Json(video_info))
}

pub async fn get_quality_options(
    Json(request): Json<QualityOptionsRequest>,
) -> Result<Json<QualityOptions>, AppError> {
    let quality_options = youtube_service::extract_quality_options(&request.url).await?;
    Ok(Json(quality_options))
}

pub async fn download_video(
    Json(request): Json<DownloadRequest>,
) -> Result<Response, AppError> {
    let file_data = youtube_service::download_video(request).await?;
    
    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/octet-stream")
        .header("Content-Disposition", "attachment; filename=\"download\"")
        .body(file_data.into())
        .unwrap();
    
    Ok(response)
}

#[derive(Debug)]
pub struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let error_message = format!("Internal server error: {}", self.0);
        (StatusCode::INTERNAL_SERVER_ERROR, error_message).into_response()
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