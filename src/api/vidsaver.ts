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
    throw new Error('Failed to fetch video information');
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
    throw new Error('Failed to fetch quality options');
  }

  return response.json();
}

export async function downloadVideo(request: DownloadRequest): Promise<void> {
  const response = await fetch(`${API_BASE_URL}/api/download`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(request),
  });

  if (!response.ok) {
    throw new Error('Download failed');
  }

  // Handle file download
  const blob = await response.blob();
  const downloadUrl = window.URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = downloadUrl;
  a.download = `download.${request.type === 'mp3' ? 'mp3' : 'mp4'}`;
  document.body.appendChild(a);
  a.click();
  document.body.removeChild(a);
  window.URL.revokeObjectURL(downloadUrl);
}