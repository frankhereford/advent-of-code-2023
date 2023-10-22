import React, { useEffect, useRef, useState } from 'react';
import Hls from 'hls.js';
import videos_of_static from "../../utils/videos_of_static";

function getStatic(): string {
  const randomIndex = Math.floor(Math.random() * videos_of_static.length);
  return videos_of_static[randomIndex]!;
}

function getRandomNumber(min: number, max: number): number {
  return Math.floor(Math.random() * (max - min + 1) + min);
}

interface VideoProps {
  video_id?: string | null;
}

const Video: React.FC<VideoProps> = ({ video_id: videoIdFromProps }) => {
  const [video_id, setVideoId] = useState<string>(videoIdFromProps ?? getStatic());
  const videoRef = useRef<HTMLVideoElement>(null);

  useEffect(() => {
    const video = videoRef.current;
    const hls = new Hls();

    const loadVideo = (url: string) => {
      if (Hls.isSupported() && video) {
        hls.loadSource(url);
        hls.attachMedia(video);
      }
    };

    if (video_id) {
      loadVideo(video_id + '/playlist.m3u8');
    }

    return () => {
      hls.destroy();
    };
  }, [video_id]);

  useEffect(() => {
    if (videoIdFromProps && videoIdFromProps !== video_id) {
      setVideoId(getStatic());
      setTimeout(() => setVideoId(videoIdFromProps), getRandomNumber(250, 750));
    }
  }, [videoIdFromProps, video_id]);

  return <video ref={videoRef} muted autoPlay loop></video>;
};

export default Video;