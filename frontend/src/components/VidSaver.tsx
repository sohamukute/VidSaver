import React, { useState } from 'react';
import { Download, Play, Music, FileAudio } from 'lucide-react';
import Header from './Header';
import UrlInput from './UrlInput';
import VideoPreview from './VideoPreview';
import DownloadSection from './DownloadSection';
import Footer from './Footer';
import { VideoInfo, QualityOptions } from '../types';
import * as api from '../api/vidsaver';

const VidSaver: React.FC = () => {
  const [videoInfo, setVideoInfo] = useState<VideoInfo | null>(null);
  const [qualityOptions, setQualityOptions] = useState<QualityOptions | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleUrlSubmit = async (url: string) => {
    setLoading(true);
    setError(null);
    setVideoInfo(null);
    setQualityOptions(null);

    try {
      const [info, qualities] = await Promise.all([
        api.getVideoInfo(url),
        api.getQualityOptions(url)
      ]);
      
      setVideoInfo(info);
      setQualityOptions(qualities);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to fetch video information');
    } finally {
      setLoading(false);
    }
  };

  const handleDownload = async (type: 'video' | 'audio' | 'mp3', videoQuality?: string, audioQuality?: string) => {
    if (!videoInfo) return;

    try {
      await api.downloadVideo({
        url: videoInfo.url,
        type,
        videoQuality,
        audioQuality
      });
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Download failed');
    }
  };

  return (
    <div className="min-h-screen text-white">
      <div className="container mx-auto px-4 py-8 max-w-4xl">
        <Header />
        
        <div className="bg-white/10 backdrop-blur-lg rounded-3xl p-8 border border-white/10 shadow-2xl mb-8">
          <UrlInput onSubmit={handleUrlSubmit} loading={loading} />
          
          {error && (
            <div className="mt-6 p-4 bg-red-500/20 border border-red-500/30 rounded-xl text-red-200">
              {error}
            </div>
          )}
          
          {videoInfo && qualityOptions && (
            <>
              <VideoPreview videoInfo={videoInfo} />
              <DownloadSection 
                qualityOptions={qualityOptions}
                onDownload={handleDownload}
              />
            </>
          )}
        </div>
        
        <Footer />
      </div>
    </div>
  );
};

export default VidSaver;