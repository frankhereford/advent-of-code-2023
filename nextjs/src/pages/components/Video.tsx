import React, { useEffect, useRef, useState } from 'react';
import Hls from 'hls.js';
import { v4 as uuidv4 } from 'uuid';
import { api } from "~/utils/api";
import { useVideos } from '~/pages/contexts/VideosContext';

import videos_of_static from "~/utils/videos_of_static";

function getStatic(): string {
  const randomIndex = Math.floor(Math.random() * videos_of_static.length);
  return videos_of_static[randomIndex]!;
}

function getRandomNumber(min: number, max: number): number {
  return Math.floor(Math.random() * (max - min + 1) + min);
}

interface VideoProps {
  videoId?: string | undefined;
}

const Video: React.FC<VideoProps> = ({ videoId: videoIdFromProps }) => {
  const [nextPlayingVideoId, setNextPlayingVideoId] = useState<string>('');
  const [playingVideoId, setPlayingVideoId] = useState<string>(videoIdFromProps ?? getStatic());
  const [uuid] = useState<string>(uuidv4());
  const videoRef = useRef<HTMLVideoElement>(null);
  const [shouldPoll, setShouldPoll] = useState(true);
  const { videosLoaded, incrementVideosLoaded, resetVideosLoaded } = useVideos();

  const randomVideo = api.media.get_random_videos.useQuery({ length: 1, uuid: uuid }, { 
    trpc: { context: { skipBatch: true, }, },
    refetchOnWindowFocus: false,
  })

  const videoReadiness = api.media.is_ready.useQuery(
    { video: videoIdFromProps },
    {
      enabled: !!videoIdFromProps && shouldPoll,
      refetchInterval: shouldPoll ? 1000 : undefined,
    }
  );

  useEffect(() => {
    if (videoReadiness.data) {
      console.info("We've received a ready, on topic video: ", videoIdFromProps)
      setNextPlayingVideoId(videoIdFromProps!);
      setShouldPoll(false);
      incrementVideosLoaded();
    }
  }, [videoReadiness.data, videoIdFromProps]);

  useEffect(() => { // handle data coming back from the random video query
    //if (videoIdFromProps) return;
    if (!randomVideo.data) return;
    setNextPlayingVideoId(randomVideo.data[0]!);
  }, [randomVideo.data, videoIdFromProps]);

  useEffect(() => { // handles changing the channel every set interval range if there is not a videoId from props
    let timer: NodeJS.Timeout;

    if (!videoIdFromProps || shouldPoll) {
      timer = setInterval(() => {
        void randomVideo.refetch();
      }, getRandomNumber(2000, 5000));
    }

    return () => {
      clearInterval(timer);
    };
  }, [videoIdFromProps, randomVideo, shouldPoll]); 


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

    return () => {
      hls.destroy();
    };
  }, [playingVideoId]);

  useEffect(() => { // this makes the static flicker when the playingVideoId changes
    if (nextPlayingVideoId && nextPlayingVideoId !== playingVideoId) {
      setPlayingVideoId(getStatic());
      setTimeout(() => setPlayingVideoId(nextPlayingVideoId), getRandomNumber(350, 950));
      setNextPlayingVideoId('');
    }
  // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [videoIdFromProps, playingVideoId, nextPlayingVideoId]);

  return <video ref={videoRef} muted autoPlay loop></video>;
};

export default Video;