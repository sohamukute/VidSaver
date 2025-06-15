import React from 'react';
import { Clock, Eye, User } from 'lucide-react';
import { VideoInfo } from '../types';

interface VideoPreviewProps {
  videoInfo: VideoInfo;
}

const VideoPreview: React.FC<VideoPreviewProps> = ({ videoInfo }) => {
  return (
    <div className="bg-black/40 rounded-2xl p-6 mb-8 border border-white/10 animate-fade-in-up">
      <div className="flex flex-col lg:flex-row gap-6 items-start">
        <img
          src={videoInfo.thumbnail}
          alt="Video Thumbnail"
          className="w-full lg:w-80 h-48 lg:h-44 object-cover rounded-xl shadow-lg"
        />
        
        <div className="flex-1 space-y-4">
          <h3 className="text-xl lg:text-2xl font-semibold text-white leading-tight">
            {videoInfo.title}
          </h3>
          
          <div className="flex flex-wrap gap-4 text-gray-300 text-sm">
            <div className="flex items-center gap-2">
              <Clock className="w-4 h-4" />
              <span>{videoInfo.duration}</span>
            </div>
            <div className="flex items-center gap-2">
              <Eye className="w-4 h-4" />
              <span>{videoInfo.views}</span>
            </div>
            <div className="flex items-center gap-2">
              <User className="w-4 h-4" />
              <span>{videoInfo.uploader}</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default VideoPreview;