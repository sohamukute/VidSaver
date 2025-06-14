use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
};
use crate::services::youtube_service;
use crate::types::*;

pub async fn get_video_info(
    Json(request): Json<VideoUrlRequest>,
) -> Result<Json<VideoInfo>, (StatusCode, Json<ApiError>)> {
    println!("Getting video info for: {}", request.url);
    
    match youtube_service::extract_video_info(&request.url).await {
        Ok(info) => Ok(Json(info)),
        Err(e) => {
            eprintln!("Error extracting video info: {}", e);
            Err((
                StatusCode::BAD_REQUEST,
                Json(ApiError {
                    error: "EXTRACTION_FAILED".to_string(),
                    message: e.to_string(),
                }),
            ))
        }
    }
}

pub async fn get_quality_options(
    Json(request): Json<VideoUrlRequest>,
) -> Result<Json<QualityOptions>, (StatusCode, Json<ApiError>)> {
    println!("Getting quality options for: {}", request.url);
    
    match youtube_service::extract_quality_options(&request.url).await {
        Ok(options) => Ok(Json(options)),
        Err(e) => {
            eprintln!("Error extracting quality options: {}", e);
            Err((
                StatusCode::BAD_REQUEST,
                Json(ApiError {
                    error: "QUALITY_EXTRACTION_FAILED".to_string(),
                    message: e.to_string(),
                }),
            ))
        }
    }
}

pub async fn download_video(
    Json(request): Json<DownloadRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<ApiError>)> {
    println!("Download request: {:?}", request);
    
    match youtube_service::download_video(request).await {
        Ok(file_data) => {
            let headers = [
                ("Content-Type", "application/octet-stream"),
                ("Content-Disposition", "attachment; filename=\"download\""),
            ];
            Ok((headers, file_data))
        }
        Err(e) => {
            eprintln!("Download error: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiError {
                    error: "DOWNLOAD_FAILED".to_string(),
                    message: e.to_string(),
                }),
            ))
        }
    }
}