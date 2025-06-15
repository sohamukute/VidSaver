import { VideoInfo, QualityOptions, DownloadRequest } from '../types';

const API_BASE_URL = 'http://localhost:3001';

export async function getVideoInfo(url: string): Promise<VideoInfo> {
  const response = await fetch(`${API_BASE_URL}/api/video-info`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ url }),
  });

  if (!response.ok) {
    const errorText = await response.text();
    throw new Error(`Failed to fetch video information: ${errorText}`);
  }

  return response.json();
}

export async function getQualityOptions(url: string): Promise<QualityOptions> {
  const response = await fetch(`${API_BASE_URL}/api/quality-options`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ url }),
  });

  if (!response.ok) {
    const errorText = await response.text();
    throw new Error(`Failed to fetch quality options: ${errorText}`);
  }

  return response.json();
}

export async function downloadVideo(request: DownloadRequest, videoTitle?: string): Promise<void> {
  const response = await fetch(`${API_BASE_URL}/api/download`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(request),
  });

  if (!response.ok) {
    const errorText = await response.text();
    throw new Error(`Download failed: ${errorText}`);
  }

  // Handle file download
  const blob = await response.blob();
  const downloadUrl = window.URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = downloadUrl;
  
  // Generate filename based on request type and video title
  const sanitizedTitle = sanitizeFilename(videoTitle || 'download');
  let filename: string;
  
  switch (request.type) {
    case 'mp3':
      filename = `${sanitizedTitle}.mp3`;
      break;
    case 'audio':
      // Use the audio format extension if available
      const audioExt = getAudioExtension(request.audioQuality);
      filename = `${sanitizedTitle}.${audioExt}`;
      break;
    case 'video':
    default:
      // Use mp4 as default for video downloads
      filename = `${sanitizedTitle}.mp4`;
      break;
  }
  
  a.download = filename;
  document.body.appendChild(a);
  a.click();
  document.body.removeChild(a);
  window.URL.revokeObjectURL(downloadUrl);
}

function sanitizeFilename(filename: string): string {
  // Remove or replace invalid filename characters
  return filename
    .replace(/[<>:"/\\|?*]/g, '_') // Replace invalid characters with underscores
    .replace(/\s+/g, '_') // Replace spaces with underscores
    .replace(/_{2,}/g, '_') // Replace multiple underscores with single
    .replace(/^_|_$/g, '') // Remove leading/trailing underscores
    .substring(0, 100); // Limit length
}

function getAudioExtension(audioQuality?: string): string {
  // Try to determine extension from format ID
  // Common YouTube audio format IDs and their extensions
  const formatExtensions: Record<string, string> = {
    '140': 'm4a',  // 128kbps AAC
    '139': 'm4a',  // 48kbps AAC
    '249': 'webm', // 50kbps Opus
    '250': 'webm', // 70kbps Opus
    '251': 'webm', // 160kbps Opus
  };
  
  if (audioQuality && formatExtensions[audioQuality]) {
    return formatExtensions[audioQuality];
  }
  
  // Default to m4a for audio downloads
  return 'm4a';
}