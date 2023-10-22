import React, { useEffect, useRef, useState } from 'react';
import Hls from 'hls.js';
import { v4 as uuidv4 } from 'uuid';
import { api } from "~/utils/api";


import videos_of_static from "~/utils/videos_of_static";

function getStatic(): string {
  const randomIndex = Math.floor(Math.random() * videos_of_static.length);
  return videos_of_static[randomIndex]!;
}

function getRandomNumber(min: number, max: number): number {
  return Math.floor(Math.random() * (max - min + 1) + min);
}

interface VideoProps {
  videoId?: string | null;
}

const Video: React.FC<VideoProps> = ({ videoId: videoIdFromProps }) => {
  const [nextPlayingVideoId, setNextPlayingVideoId] = useState<string>('');
  const [playingVideoId, setPlayingVideoId] = useState<string>(videoIdFromProps ?? getStatic());
  const [inLoading, setInLoading] = useState<boolean>(false);
  const videoRef = useRef<HTMLVideoElement>(null);
  const [uuid] = useState<string>(uuidv4());

  const randomVideo = api.media.get_random_videos.useQuery({ length: 1, uuid: uuid }, { 
    refetchOnWindowFocus: false,
  })

  useEffect(() => { // handle data coming back from the random video query
    if (videoIdFromProps) return;
    if (!randomVideo.data) return;
    setNextPlayingVideoId(randomVideo.data[0]!);
  }, [randomVideo.data, videoIdFromProps]);

  useEffect(() => { // handles changing the channel ever 10-20 seconds if there is not a videoId from props
    let timer: NodeJS.Timeout;

    if (!videoIdFromProps) {
      timer = setInterval(() => {
        void randomVideo.refetch();
      }, getRandomNumber(3000, 6000));
    }

    return () => {
      clearInterval(timer);
    };
  }, [videoIdFromProps, randomVideo]); 


  useEffect(() => { // this handles setting up the HLS video player
    const video = videoRef.current;
    const hls = new Hls();

    const loadVideo = (url: string) => {
      if (Hls.isSupported() && video) {
        hls.loadSource(url);
        hls.attachMedia(video);
      }
    };

    if (playingVideoId) {
      loadVideo(playingVideoId + '/playlist.m3u8');
    }

    setInLoading(false);

    return () => {
      hls.destroy();
    };

  }, [playingVideoId]);

  useEffect(() => { // this makes the static flicker when the playingVideoId changes
    if (inLoading) return;
    if (videoIdFromProps && videoIdFromProps !== playingVideoId) {
      setPlayingVideoId(getStatic());
      console.log("got a new video assignment", videoIdFromProps);
      setTimeout(() => setPlayingVideoId(videoIdFromProps), getRandomNumber(250, 750));
      setInLoading(true);
    }
    else if (nextPlayingVideoId && nextPlayingVideoId !== playingVideoId) {
      setPlayingVideoId(getStatic());
      setTimeout(() => setPlayingVideoId(nextPlayingVideoId), getRandomNumber(250, 750));
      setNextPlayingVideoId('');
      setInLoading(true);
    }
  }, [videoIdFromProps, playingVideoId, nextPlayingVideoId]);

  return <video ref={videoRef} muted autoPlay loop></video>;
};

export default Video;