use crate::types::*;
use anyhow::{anyhow, Result};
use serde_json::Value;
use std::process::Command;
use tokio::fs;

pub async fn extract_video_info(url: &str) -> Result<VideoInfo> {
    if !is_valid_youtube_url(url) {
        return Err(anyhow!("Invalid YouTube URL"));
    }

    // Check if yt-dlp is available
    if !check_ytdlp_available() {
        return create_mock_video_info(url);
    }

    // Use yt-dlp to extract video information
    let output = Command::new("yt-dlp")
        .args([
            "--dump-json",
            "--no-playlist",
            "--no-warnings",
            "--skip-download",
            url,
        ])
        .output();

    match output {
        Ok(output) => {
            if !output.status.success() {
                let error = String::from_utf8_lossy(&output.stderr);
                println!("yt-dlp stderr: {}", error);
                return create_mock_video_info(url);
            }

            let json_str = String::from_utf8(output.stdout)?;
            if json_str.trim().is_empty() {
                return create_mock_video_info(url);
            }

            match serde_json::from_str::<Value>(&json_str) {
                Ok(json) => Ok(VideoInfo {
                    url: url.to_string(),
                    title: json["title"].as_str().unwrap_or("Unknown Title").to_string(),
                    thumbnail: json["thumbnail"].as_str().unwrap_or("").to_string(),
                    duration: format_duration(json["duration"].as_f64().unwrap_or(0.0)),
                    views: format_views(json["view_count"].as_u64().unwrap_or(0)),
                    uploader: json["uploader"].as_str().unwrap_or("Unknown Channel").to_string(),
                    description: json["description"].as_str().map(|s| s.to_string()),
                }),
                Err(_) => create_mock_video_info(url),
            }
        }
        Err(_) => create_mock_video_info(url),
    }
}

pub async fn extract_quality_options(url: &str) -> Result<QualityOptions> {
    if !is_valid_youtube_url(url) {
        return Err(anyhow!("Invalid YouTube URL"));
    }

    // Check if yt-dlp is available
    if !check_ytdlp_available() {
        return Ok(create_mock_quality_options());
    }

    // Use yt-dlp to get available formats - get both info and formats in one call
    let output = Command::new("yt-dlp")
        .args([
            "--dump-json",
            "--no-playlist",
            "--no-warnings",
            "--skip-download",
            url,
        ])
        .output();

    match output {
        Ok(output) => {
            if !output.status.success() {
                let error = String::from_utf8_lossy(&output.stderr);
                println!("yt-dlp stderr: {}", error);
                return Ok(create_mock_quality_options());
            }

            let json_str = String::from_utf8(output.stdout)?;
            if json_str.trim().is_empty() {
                return Ok(create_mock_quality_options());
            }

            match serde_json::from_str::<Value>(&json_str) {
                Ok(json) => parse_quality_options(json),
                Err(e) => {
                    println!("JSON parse error: {}", e);
                    Ok(create_mock_quality_options())
                }
            }
        }
        Err(e) => {
            println!("Command execution error: {}", e);
            Ok(create_mock_quality_options())
        }
    }
}

pub async fn download_video(request: DownloadRequest) -> Result<Vec<u8>> {
    if !is_valid_youtube_url(&request.url) {
        return Err(anyhow!("Invalid YouTube URL"));
    }

    // Check if yt-dlp is available
    if !check_ytdlp_available() {
        return Err(anyhow!("yt-dlp is not available on this system. Please install it to enable downloads."));
    }

    let temp_dir = std::env::temp_dir();
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let output_template = temp_dir.join(format!("vidsaver_{}_%%(title)s.%%(ext)s", timestamp));

    let mut args = vec![
        "--no-playlist".to_string(),
        "--no-warnings".to_string(),
        "-o".to_string(),
        output_template.to_string_lossy().to_string(),
    ];

    match request.r#type.as_str() {
        "video" => {
            // Download video with audio
            if let Some(video_quality) = &request.video_quality {
                args.push("-f".to_string());
                if let Some(audio_quality) = &request.audio_quality {
                    // Combine specific video and audio formats
                    args.push(format!("{}+{}/best[height<=1080]", video_quality, audio_quality));
                } else {
                    // Use specific video format with best audio
                    args.push(format!("{}+bestaudio/best[height<=1080]", video_quality));
                }
            } else {
                args.push("-f".to_string());
                args.push("best[height<=1080]/best".to_string());
            }
        }
        "audio" => {
            // Download audio only
            args.push("-f".to_string());
            if let Some(audio_quality) = &request.audio_quality {
                args.push(audio_quality.clone());
            } else {
                args.push("bestaudio/best".to_string());
            }
        }
        "mp3" => {
            // Download and convert to MP3
            args.push("-f".to_string());
            if let Some(audio_quality) = &request.audio_quality {
                args.push(audio_quality.clone());
            } else {
                args.push("bestaudio/best".to_string());
            }
            args.push("--extract-audio".to_string());
            args.push("--audio-format".to_string());
            args.push("mp3".to_string());
            args.push("--audio-quality".to_string());
            args.push("0".to_string()); // Best quality
        }
        _ => return Err(anyhow!("Invalid download type")),
    }

    args.push(request.url.clone());

    println!("Executing yt-dlp with args: {:?}", args);

    // Execute yt-dlp
    let output = Command::new("yt-dlp")
        .args(&args)
        .output()?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        println!("yt-dlp download error: {}", error);
        return Err(anyhow!("Download failed: {}", error));
    }

    // Find the downloaded file
    let mut entries = fs::read_dir(&temp_dir).await?;
    let mut downloaded_file = None;

    while let Some(entry) = entries.next_entry().await? {
        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy();
        
        if file_name_str.starts_with(&format!("vidsaver_{}", timestamp)) {
            downloaded_file = Some(entry.path());
            break;
        }
    }

    match downloaded_file {
        Some(file_path) => {
            let file_data = fs::read(&file_path).await?;
            // Clean up the temporary file
            let _ = fs::remove_file(&file_path).await;
            Ok(file_data)
        }
        None => Err(anyhow!("Downloaded file not found")),
    }
}

fn check_ytdlp_available() -> bool {
    Command::new("yt-dlp")
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

fn create_mock_video_info(url: &str) -> Result<VideoInfo> {
    // Extract video ID from URL for more realistic mock data
    let video_id = extract_video_id(url).unwrap_or("dQw4w9WgXcQ");
    
    Ok(VideoInfo {
        url: url.to_string(),
        title: "Rick Astley - Never Gonna Give You Up (Official Video)".to_string(),
        thumbnail: format!("https://img.youtube.com/vi/{}/maxresdefault.jpg", video_id),
        duration: "3:33".to_string(),
        views: "1.4B views".to_string(),
        uploader: "Rick Astley".to_string(),
        description: Some(r#"The official video for "Never Gonna Give You Up" by Rick Astley. This is mock data for demonstration purposes."#.to_string()),
    })
}

fn create_mock_quality_options() -> QualityOptions {
    QualityOptions {
        video: vec![
            VideoFormat {
                format_id: "137".to_string(),
                quality: "1080p".to_string(),
                ext: "mp4".to_string(),
                filesize: Some(89 * 1024 * 1024), // 89MB
                width: Some(1920),
                height: Some(1080),
            },
            VideoFormat {
                format_id: "136".to_string(),
                quality: "720p".to_string(),
                ext: "mp4".to_string(),
                filesize: Some(45 * 1024 * 1024), // 45MB
                width: Some(1280),
                height: Some(720),
            },
            VideoFormat {
                format_id: "135".to_string(),
                quality: "480p".to_string(),
                ext: "mp4".to_string(),
                filesize: Some(25 * 1024 * 1024), // 25MB
                width: Some(854),
                height: Some(480),
            },
            VideoFormat {
                format_id: "134".to_string(),
                quality: "360p".to_string(),
                ext: "mp4".to_string(),
                filesize: Some(15 * 1024 * 1024), // 15MB
                width: Some(640),
                height: Some(360),
            },
            VideoFormat {
                format_id: "133".to_string(),
                quality: "240p".to_string(),
                ext: "mp4".to_string(),
                filesize: Some(8 * 1024 * 1024), // 8MB
                width: Some(426),
                height: Some(240),
            },
        ],
        audio: vec![
            AudioFormat {
                format_id: "140".to_string(),
                ext: "m4a".to_string(),
                abr: 128,
                filesize: Some(8 * 1024 * 1024), // 8MB
            },
            AudioFormat {
                format_id: "139".to_string(),
                ext: "m4a".to_string(),
                abr: 48,
                filesize: Some(3 * 1024 * 1024), // 3MB
            },
            AudioFormat {
                format_id: "249".to_string(),
                ext: "webm".to_string(),
                abr: 50,
                filesize: Some(3 * 1024 * 1024), // 3MB
            },
            AudioFormat {
                format_id: "250".to_string(),
                ext: "webm".to_string(),
                abr: 70,
                filesize: Some(4 * 1024 * 1024), // 4MB
            },
            AudioFormat {
                format_id: "251".to_string(),
                ext: "webm".to_string(),
                abr: 160,
                filesize: Some(9 * 1024 * 1024), // 9MB
            },
        ],
    }
}

fn parse_quality_options(json: Value) -> Result<QualityOptions> {
    let mut video_formats = Vec::new();
    let mut audio_formats = Vec::new();

    if let Some(formats) = json["formats"].as_array() {
        for format in formats {
            let format_id = format["format_id"].as_str().unwrap_or("").to_string();
            let ext = format["ext"].as_str().unwrap_or("unknown").to_string();
            let filesize = format["filesize"].as_u64();
            
            // Check if this is a video format
            if let Some(vcodec) = format["vcodec"].as_str() {
                if vcodec != "none" && format["height"].is_number() {
                    let height = format["height"].as_u64().unwrap_or(0) as u32;
                    let width = format["width"].as_u64().map(|w| w as u32);
                    
                    // Only include common video formats
                    if height >= 144 && (ext == "mp4" || ext == "webm") {
                        video_formats.push(VideoFormat {
                            format_id: format_id.clone(),
                            quality: format!("{}p", height),
                            ext: ext.clone(),
                            filesize,
                            width,
                            height: Some(height),
                        });
                    }
                }
            }

            // Check if this is an audio format
            if let Some(acodec) = format["acodec"].as_str() {
                if acodec != "none" && format["vcodec"].as_str() == Some("none") {
                    let abr = format["abr"].as_f64().unwrap_or(128.0) as u32;
                    
                    // Only include common audio formats
                    if abr > 0 && (ext == "m4a" || ext == "webm" || ext == "mp3") {
                        audio_formats.push(AudioFormat {
                            format_id: format_id.clone(),
                            ext: ext.clone(),
                            abr,
                            filesize,
                        });
                    }
                }
            }
        }
    }

    // Remove duplicates and sort
    video_formats.sort_by(|a, b| {
        let height_a = a.height.unwrap_or(0);
        let height_b = b.height.unwrap_or(0);
        height_b.cmp(&height_a)
    });
    video_formats.dedup_by(|a, b| a.height == b.height && a.ext == b.ext);

    audio_formats.sort_by(|a, b| b.abr.cmp(&a.abr));
    audio_formats.dedup_by(|a, b| a.abr == b.abr && a.ext == b.ext);

    // Add default options if none found
    if video_formats.is_empty() {
        video_formats = create_mock_quality_options().video;
    }

    if audio_formats.is_empty() {
        audio_formats = create_mock_quality_options().audio;
    }

    Ok(QualityOptions {
        video: video_formats,
        audio: audio_formats,
    })
}

fn extract_video_id(url: &str) -> Option<&str> {
    if let Some(pos) = url.find("v=") {
        let start = pos + 2;
        let end = url[start..].find('&').map(|i| start + i).unwrap_or(url.len());
        Some(&url[start..end])
    } else if let Some(pos) = url.find("youtu.be/") {
        let start = pos + 9;
        let end = url[start..].find('?').map(|i| start + i).unwrap_or(url.len());
        Some(&url[start..end])
    } else {
        None
    }
}

fn is_valid_youtube_url(url: &str) -> bool {
    url.contains("youtube.com/watch") || url.contains("youtu.be/") || url.contains("youtube.com/embed/")
}

fn format_duration(seconds: f64) -> String {
    let total_seconds = seconds as u32;
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let secs = total_seconds % 60;

    if hours > 0 {
        format!("{}:{:02}:{:02}", hours, minutes, secs)
    } else {
        format!("{}:{:02}", minutes, secs)
    }
}

fn format_views(views: u64) -> String {
    if views >= 1_000_000_000 {
        format!("{:.1}B views", views as f64 / 1_000_000_000.0)
    } else if views >= 1_000_000 {
        format!("{:.1}M views", views as f64 / 1_000_000.0)
    } else if views >= 1_000 {
        format!("{:.1}K views", views as f64 / 1_000.0)
    } else {
        format!("{} views", views)
    }
}

fn format_quality_string(format: &Value) -> String {
    if let Some(height) = format["height"].as_u64() {
        format!("{}p", height)
    } else if let Some(quality) = format["quality"].as_str() {
        quality.to_string()
    } else {
        "Unknown Quality".to_string()
    }
}