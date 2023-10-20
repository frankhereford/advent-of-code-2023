import React, { useEffect, useRef, useState } from 'react';
import Hls from 'hls.js';
import videos_of_static from "../utils/videos_of_static"

function getStatic(): string {
  const randomIndex = Math.floor(Math.random() * videos_of_static.length);
  return videos_of_static[randomIndex]!;
}

interface VideoComponentProps {
  video_id?: string | null;
}

const VideoComponent: React.FC<VideoComponentProps> = ({ video_id: videoIdFromProps }) => {
  const [video_id] = useState<string>(videoIdFromProps ?? getStatic());
  const videoRef = useRef<HTMLVideoElement>(null);

  useEffect(() => {
    if (!video_id) return;

    const video = videoRef.current;
    if (Hls.isSupported() && video) {
      const hls = new Hls();
      const url = video_id + '/playlist.m3u8';
      hls.loadSource(url);
      hls.attachMedia(video);
    }
  }, [video_id]);

  return <video ref={videoRef} muted autoPlay loop></video>;
};

export default VideoComponent;
