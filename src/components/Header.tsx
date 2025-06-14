import React from 'react';
import { Download } from 'lucide-react';

const Header: React.FC = () => {
  return (
    <div className="text-center mb-12 py-8">
      <div className="flex items-center justify-center gap-3 mb-4">
        <Download className="w-12 h-12 text-red-400" />
        <h1 className="text-5xl font-bold bg-gradient-to-r from-red-400 via-red-300 to-pink-400 bg-clip-text text-transparent">
          VidSaver
        </h1>
      </div>
      <p className="text-xl text-gray-300 font-light">
        Download YouTube videos in the format you love
      </p>
    </div>
  );
};

export default Header;