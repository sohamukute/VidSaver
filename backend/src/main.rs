use axum::{
    routing::{get, post},
    Router,
};
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber;

mod handlers;
mod services;
mod types;

use handlers::*;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Build our application with routes
    let app = Router::new()
        .route("/", get(health_check))
        .route("/api/video-info", post(get_video_info))
        .route("/api/quality-options", post(get_quality_options))
        .route("/api/download", post(download_video))
        .layer(
            ServiceBuilder::new().layer(
                CorsLayer::new()
                    .allow_origin(Any)
                    .allow_methods(Any)
                    .allow_headers(Any),
            ),
        );

    // Run it with hyper on localhost:3001
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    println!("🚀 VidSaver backend running on http://localhost:3001");
    println!("📋 Available endpoints:");
    println!("  GET  / - Health check");
    println!("  POST /api/video-info - Extract video metadata");
    println!("  POST /api/quality-options - Get available qualities");
    println!("  POST /api/download - Download video/audio");
    
    // Check if yt-dlp is available
    if std::process::Command::new("yt-dlp").arg("--version").output().is_ok() {
        println!("✅ yt-dlp is available");
    } else {
        println!("⚠️  yt-dlp is not available - only mock data will be returned");
    }
    
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> &'static str {
    "VidSaver Backend is running! 🎬"
}