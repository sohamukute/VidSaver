export interface VideoInfo {
  url: string;
  title: string;
  thumbnail: string;
  duration: string;
  views: string;
  uploader: string;
  description?: string;
}

export interface VideoFormat {
  format_id: string;
  quality: string;
  ext: string;
  filesize?: number;
  width?: number;
  height?: number;
}

export interface AudioFormat {
  format_id: string;
  ext: string;
  abr: number;
  filesize?: number;
}

export interface QualityOptions {
  video: VideoFormat[];
  audio: AudioFormat[];
}

export interface DownloadRequest {
  url: string;
  type: 'video' | 'audio' | 'mp3';
  videoQuality?: string;
  audioQuality?: string;
}