import React, { useState } from 'react';
import { Download, Music, FileAudio, Loader2, CheckCircle, Info } from 'lucide-react';
import { QualityOptions } from '../types';

interface DownloadSectionProps {
  qualityOptions: QualityOptions;
  onDownload: (type: 'video' | 'audio' | 'mp3', videoQuality?: string, audioQuality?: string) => void;
}

const DownloadSection: React.FC<DownloadSectionProps> = ({ qualityOptions, onDownload }) => {
  const [videoQuality, setVideoQuality] = useState(qualityOptions.video[0]?.format_id || '');
  const [audioQuality, setAudioQuality] = useState(qualityOptions.audio[0]?.format_id || '');
  const [downloadingStates, setDownloadingStates] = useState<Record<string, boolean>>({});
  const [completedStates, setCompletedStates] = useState<Record<string, boolean>>({});

  const handleDownload = async (type: 'video' | 'audio' | 'mp3') => {
    setDownloadingStates(prev => ({ ...prev, [type]: true }));
    setCompletedStates(prev => ({ ...prev, [type]: false }));

    try {
      await onDownload(type, videoQuality, audioQuality);
      
      // Simulate download completion
      setTimeout(() => {
        setDownloadingStates(prev => ({ ...prev, [type]: false }));
        setCompletedStates(prev => ({ ...prev, [type]: true }));
        
        // Reset completed state after 3 seconds
        setTimeout(() => {
          setCompletedStates(prev => ({ ...prev, [type]: false }));
        }, 3000);
      }, 2000);
    } catch (error) {
      setDownloadingStates(prev => ({ ...prev, [type]: false }));
    }
  };

  const getButtonContent = (type: string, icon: React.ReactNode, text: string) => {
    if (downloadingStates[type]) {
      return (
        <>
          <Loader2 className="w-5 h-5 animate-spin" />
          Preparing...
        </>
      );
    }
    
    if (completedStates[type]) {
      return (
        <>
          <CheckCircle className="w-5 h-5" />
          Ready to Download
        </>
      );
    }
    
    return (
      <>
        {icon}
        {text}
      </>
    );
  };

  const formatFileSize = (bytes?: number) => {
    if (!bytes) return 'Size unknown';
    const mb = bytes / (1024 * 1024);
    return `${mb.toFixed(1)}MB`;
  };

  const getSelectedVideoFormat = () => {
    return qualityOptions.video.find(v => v.format_id === videoQuality);
  };

  const getSelectedAudioFormat = () => {
    return qualityOptions.audio.find(a => a.format_id === audioQuality);
  };

  return (
    <div className="space-y-6">
      {/* Format Information */}
      <div className="bg-blue-500/10 border border-blue-500/20 rounded-xl p-4">
        <div className="flex items-center gap-2 mb-2">
          <Info className="w-5 h-5 text-blue-400" />
          <span className="text-blue-300 font-semibold">Available Formats</span>
        </div>
        <div className="text-sm text-gray-300">
          <p>Video formats: {qualityOptions.video.length} options available</p>
          <p>Audio formats: {qualityOptions.audio.length} options available</p>
        </div>
      </div>

      {/* Quality Selection */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        <div className="bg-white/5 rounded-xl p-5 border border-white/10">
          <label className="block text-red-300 font-semibold mb-3">
            Video Quality ({qualityOptions.video.length} options)
          </label>
          <select
            value={videoQuality}
            onChange={(e) => setVideoQuality(e.target.value)}
            className="w-full px-4 py-3 bg-black/30 border border-white/20 rounded-lg text-white focus:outline-none focus:border-red-400 focus:shadow-lg focus:shadow-red-400/10 transition-all duration-300"
          >
            {qualityOptions.video.map((option) => (
              <option key={option.format_id} value={option.format_id} className="bg-gray-800">
                {option.quality} - {option.ext?.toUpperCase()} 
                {option.width && option.height && ` (${option.width}x${option.height})`}
                {' - '}{formatFileSize(option.filesize)}
              </option>
            ))}
          </select>
          {getSelectedVideoFormat() && (
            <div className="mt-2 text-xs text-gray-400">
              Selected: {getSelectedVideoFormat()?.quality} • {getSelectedVideoFormat()?.ext?.toUpperCase()} • {formatFileSize(getSelectedVideoFormat()?.filesize)}
            </div>
          )}
        </div>

        <div className="bg-white/5 rounded-xl p-5 border border-white/10">
          <label className="block text-red-300 font-semibold mb-3">
            Audio Quality ({qualityOptions.audio.length} options)
          </label>
          <select
            value={audioQuality}
            onChange={(e) => setAudioQuality(e.target.value)}
            className="w-full px-4 py-3 bg-black/30 border border-white/20 rounded-lg text-white focus:outline-none focus:border-red-400 focus:shadow-lg focus:shadow-red-400/10 transition-all duration-300"
          >
            {qualityOptions.audio.map((option) => (
              <option key={option.format_id} value={option.format_id} className="bg-gray-800">
                {option.abr}kbps - {option.ext?.toUpperCase()} - {formatFileSize(option.filesize)}
              </option>
            ))}
          </select>
          {getSelectedAudioFormat() && (
            <div className="mt-2 text-xs text-gray-400">
              Selected: {getSelectedAudioFormat()?.abr}kbps • {getSelectedAudioFormat()?.ext?.toUpperCase()} • {formatFileSize(getSelectedAudioFormat()?.filesize)}
            </div>
          )}
        </div>
      </div>

      {/* Download Preview */}
      <div className="bg-white/5 rounded-xl p-5 border border-white/10">
        <h4 className="text-white font-semibold mb-3">Download Preview</h4>
        <div className="grid grid-cols-1 md:grid-cols-3 gap-4 text-sm">
          <div className="bg-red-500/10 border border-red-500/20 rounded-lg p-3">
            <div className="text-red-300 font-medium mb-1">Video + Audio</div>
            <div className="text-gray-300">
              {getSelectedVideoFormat()?.quality} + {getSelectedAudioFormat()?.abr}kbps
            </div>
            <div className="text-gray-400 text-xs">
              Est. size: {formatFileSize((getSelectedVideoFormat()?.filesize || 0) + (getSelectedAudioFormat()?.filesize || 0))}
            </div>
          </div>
          <div className="bg-purple-500/10 border border-purple-500/20 rounded-lg p-3">
            <div className="text-purple-300 font-medium mb-1">Audio Only</div>
            <div className="text-gray-300">
              {getSelectedAudioFormat()?.abr}kbps {getSelectedAudioFormat()?.ext?.toUpperCase()}
            </div>
            <div className="text-gray-400 text-xs">
              Est. size: {formatFileSize(getSelectedAudioFormat()?.filesize)}
            </div>
          </div>
          <div className="bg-cyan-500/10 border border-cyan-500/20 rounded-lg p-3">
            <div className="text-cyan-300 font-medium mb-1">MP3 Conversion</div>
            <div className="text-gray-300">
              High Quality MP3
            </div>
            <div className="text-gray-400 text-xs">
              Converted from {getSelectedAudioFormat()?.ext?.toUpperCase()}
            </div>
          </div>
        </div>
      </div>

      {/* Download Buttons */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
        <button
          onClick={() => handleDownload('video')}
          disabled={downloadingStates.video}
          className="flex items-center justify-center gap-3 px-6 py-4 bg-gradient-to-r from-red-500 to-red-600 text-white font-semibold rounded-xl shadow-lg hover:shadow-xl hover:shadow-red-500/25 hover:-translate-y-0.5 transition-all duration-300 disabled:opacity-70 disabled:cursor-not-allowed disabled:transform-none"
        >
          {getButtonContent('video', <Download className="w-5 h-5" />, 'Video + Audio')}
        </button>

        <button
          onClick={() => handleDownload('audio')}
          disabled={downloadingStates.audio}
          className="flex items-center justify-center gap-3 px-6 py-4 bg-gradient-to-r from-purple-500 to-purple-600 text-white font-semibold rounded-xl shadow-lg hover:shadow-xl hover:shadow-purple-500/25 hover:-translate-y-0.5 transition-all duration-300 disabled:opacity-70 disabled:cursor-not-allowed disabled:transform-none"
        >
          {getButtonContent('audio', <FileAudio className="w-5 h-5" />, 'Audio Only')}
        </button>

        <button
          onClick={() => handleDownload('mp3')}
          disabled={downloadingStates.mp3}
          className="flex items-center justify-center gap-3 px-6 py-4 bg-gradient-to-r from-cyan-500 to-blue-500 text-white font-semibold rounded-xl shadow-lg hover:shadow-xl hover:shadow-cyan-500/25 hover:-translate-y-0.5 transition-all duration-300 disabled:opacity-70 disabled:cursor-not-allowed disabled:transform-none"
        >
          {getButtonContent('mp3', <Music className="w-5 h-5" />, 'Convert to MP3')}
        </button>
      </div>
    </div>
  );
};

export default DownloadSection;