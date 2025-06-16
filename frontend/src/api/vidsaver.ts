import { VideoInfo, QualityOptions, DownloadRequest } from '../types';

const API_BASE_URL = import.meta.env.VITE_API_URL;

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

export async function downloadVideo(request: DownloadRequest): Promise<void> {
  console.log('Downloading with request:', request);
  
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

  // Get filename from Content-Disposition header if available
  const contentDisposition = response.headers.get('content-disposition');
  let filename = 'download';
  
  if (contentDisposition) {
    const filenameMatch = contentDisposition.match(/filename="?([^"]+)"?/);
    if (filenameMatch) {
      filename = filenameMatch[1];
    }
  } else {
    // Fallback filename based on type
    const extension = request.type === 'mp3' ? 'mp3' : 
                     request.type === 'audio' ? 'm4a' : 'mp4';
    filename = `download.${extension}`;
  }

  // Handle file download
  const blob = await response.blob();
  const downloadUrl = window.URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = downloadUrl;
  a.download = filename;
  a.style.display = 'none';
  document.body.appendChild(a);
  a.click();
  document.body.removeChild(a);
  window.URL.revokeObjectURL(downloadUrl);
  
  console.log(`Download completed: ${filename} (${blob.size} bytes)`);
}