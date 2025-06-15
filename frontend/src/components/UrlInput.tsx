import React, { useState } from 'react';
import { Play, Loader2 } from 'lucide-react';

interface UrlInputProps {
  onSubmit: (url: string) => void;
  loading: boolean;
}

const UrlInput: React.FC<UrlInputProps> = ({ onSubmit, loading }) => {
  const [url, setUrl] = useState('');

  const isValidYouTubeUrl = (url: string): boolean => {
    const youtubeRegex = /^(https?:\/\/)?(www\.)?(youtube\.com\/(watch\?v=|embed\/|v\/)|youtu\.be\/)[\w-]+/;
    return youtubeRegex.test(url);
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (url.trim() && isValidYouTubeUrl(url.trim())) {
      onSubmit(url.trim());
    }
  };

  const isValid = url.trim() && isValidYouTubeUrl(url.trim());

  return (
    <form onSubmit={handleSubmit} className="flex flex-col sm:flex-row gap-4 mb-8">
      <input
        type="text"
        value={url}
        onChange={(e) => setUrl(e.target.value)}
        placeholder="Paste your YouTube URL here..."
        className="flex-1 px-6 py-4 bg-black/30 border-2 border-white/10 rounded-2xl text-white placeholder-gray-400 focus:outline-none focus:border-red-400 focus:shadow-lg focus:shadow-red-400/20 transition-all duration-300 backdrop-blur-sm"
        disabled={loading}
      />
      <button
        type="submit"
        disabled={!isValid || loading}
        className="px-8 py-4 bg-gradient-to-r from-red-500 to-red-600 text-white font-semibold rounded-2xl shadow-lg hover:shadow-xl hover:shadow-red-500/25 hover:-translate-y-0.5 transition-all duration-300 disabled:opacity-50 disabled:cursor-not-allowed disabled:transform-none flex items-center gap-2 uppercase tracking-wide"
      >
        {loading ? (
          <>
            <Loader2 className="w-5 h-5 animate-spin" />
            Processing...
          </>
        ) : (
          <>
            <Play className="w-5 h-5" />
            Proceed
          </>
        )}
      </button>
    </form>
  );
};

export default UrlInput;