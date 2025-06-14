import React from 'react';
import { Heart, Mail } from 'lucide-react';

const Footer: React.FC = () => {
  return (
    <div className="text-center py-8 border-t border-white/10">
      <div className="space-y-4">
        <div className="flex items-center justify-center gap-2 text-red-300 text-lg">
          <Heart className="w-5 h-5 fill-current" />
          <span>Made with love for people</span>
        </div>
        
        <div className="flex items-center justify-center gap-2 text-gray-400">
          <Mail className="w-4 h-4" />
          <span>Having issues? Contact us at support@vidsaver.com</span>
        </div>
      </div>
    </div>
  );
};

export default Footer;