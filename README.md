# VidSaver - YouTube Video Downloader

A modern, full-stack YouTube video downloader built with React (TypeScript) frontend and Rust (Axum) backend.

## Features

- 🎬 Extract real YouTube video metadata (title, thumbnail, duration, views, channel)
- 🎯 Dynamic quality selection based on available formats
- 📱 Responsive design with glassmorphism UI
- ⬇️ Multiple download options (Video+Audio, Audio Only, MP3 conversion)
- 🚀 Fast Rust backend with concurrent processing
- 🎨 Beautiful animations and hover effects
## Video
https://github.com/sohamukute/VidSaver/blob/main/assets/demo.mp4)

## Tech Stack

### Frontend
- React 18 with TypeScript
- Tailwind CSS for styling
- Lucide React for icons
- Vite for development and building

### Backend
- Rust with Axum web framework
- yt-dlp for video metadata extraction and downloading
- FFmpeg integration for format conversion
- Async/await for concurrent downloads

## Prerequisites

Make sure you have installed:
- Node.js (v16 or later)
- Rust (latest stable version)
- yt-dlp (`pip install yt-dlp` or `brew install yt-dlp`)
- FFmpeg (`brew install ffmpeg` or download from https://ffmpeg.org/)

## Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd vidsaver
```

2. Install frontend dependencies:
```bash
npm install
```

3. Build the Rust backend:
```bash
cd backend
cargo build --release
cd ..
```

## Running the Application

1. Start the backend server:
```bash
npm run 
```
The backend will run on http://localhost:3001

2. In a new terminal, start the frontend development server:
```bash
npm run dev
```
The frontend will run on http://localhost:5173

## Usage

1. Open your browser and navigate to http://localhost:5173
2. Paste a YouTube URL in the input field
3. Click "Proceed" to fetch video information
4. Select your preferred video and audio quality
5. Choose your download option:
   - **Video + Audio**: Download complete video file
   - **Audio Only**: Download audio track only
   - **Convert to MP3**: Download and convert to MP3 format

## API Endpoints

- `POST /api/video-info` - Extract video metadata
- `POST /api/quality-options` - Get available quality options
- `POST /api/download` - Download video in specified format

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

## License

This project is for educational purposes only. Please respect YouTube's Terms of Service and copyright laws.

## Support

For issues or questions, contact: support@vidsaver.com
