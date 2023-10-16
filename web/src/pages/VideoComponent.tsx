import React, { useEffect, useRef } from 'react';
import Hls from 'hls.js';

const VideoComponent: React.FC = () => {
  const videoRef = useRef<HTMLVideoElement | null>(null);

  useEffect(() => {
    const video = videoRef.current;
    if (Hls.isSupported() && video) {
      const hls = new Hls();
      hls.loadSource('/video_processor/lzTQdb1hYBI/playlist.m3u8');
      hls.attachMedia(video);
    }
  }, []);

  return <video ref={videoRef} muted autoPlay></video>;
};

export default VideoComponent;
