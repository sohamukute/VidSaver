use axum::{
    routing::{get, post},
    Router,
};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber;
use std::process::Command;

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

    // Run it with hyper on the port Render provides
    let port = std::env::var("PORT").unwrap_or_else(|_| "3001".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr).await.unwrap();

    println!("🚀 VidSaver backend running on http://localhost:{port}");
    println!("📋 Available endpoints:");
    println!("  GET  / - Health check");
    println!("  POST /api/video-info - Extract video metadata");
    println!("  POST /api/quality-options - Get available qualities");
    println!("  POST /api/download - Download video/audio");

    // Check if yt-dlp is available
    let yt_dlp_path = std::env::var("YT_DLP_PATH").unwrap_or_else(|_| "yt-dlp".to_string());

    let yt_dlp_available = Command::new(&yt_dlp_path)
        .arg("--version")
        .output()
        .is_ok();

    if yt_dlp_available {
        println!("✅ yt-dlp is available at {}", yt_dlp_path);
    } else {
        println!("⚠️ yt-dlp not found. Attempting to install via pip...");

        let install_result = Command::new("python3")
            .args(["-m", "pip", "install", "--user", "yt-dlp"])
            .status();

        match install_result {
            Ok(status) if status.success() => {
                println!("✅ yt-dlp installed via pip");
            }
            _ => {
                println!("❌ Failed to install yt-dlp via pip");
            }
        }
    }

    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> &'static str {
    "VidSaver Backend is running! 🎬"
}
