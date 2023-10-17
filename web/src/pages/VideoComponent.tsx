import React, { useEffect, useRef } from 'react';
import Hls from 'hls.js';

interface VideoComponentProps {
  video_id?: string | null;
}

const VideoComponent: React.FC<VideoComponentProps> = ({ video_id }) => {
  const videoRef = useRef<HTMLVideoElement | null>(null);

  useEffect(() => {
    if (!video_id) return;

    const video = videoRef.current;
    if (Hls.isSupported() && video) {
      const hls = new Hls();
      const url = '/video_processor/' + video_id + '/playlist.m3u8'
      hls.loadSource(url);
      hls.attachMedia(video);
    }
  }, [video_id]);

  return <video ref={videoRef} muted autoPlay loop></video>;
};

export default VideoComponent;
