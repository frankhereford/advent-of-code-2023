import { useEffect, useState } from 'react';
import Head from "next/head";

import VideoComponent from './VideoComponent'; 

import { api } from "~/utils/api";
export default function Home() {
  
  //const [videoReady, setVideoReady] = useState({});
  const [videoReady, setVideoReady] = useState<Record<string, boolean>>({});

  //const [videoIDs, setVideoIDs] = useState([]);
  const [videoIDs, setVideoIDs] = useState<string[]>([]);


  const topic = api.openai.get_topic.useQuery();
  const videos = api.youtube.get_video.useQuery({ topic: topic.data!?.topic }, { enabled: !!topic.data });

  useEffect(() => {
    if (!videos.data) return;

    let isCancelled = false;

    const newVideoIDs = videos.data.videos;
    setVideoIDs(newVideoIDs);

    const checkVideos = async () => {
      for (const videoID of newVideoIDs) {
        // Skip if this video is already ready
        if (videoReady[videoID]) continue;

        const url = `/video_processor/${videoID}/playlist.m3u8`;
        const res = await fetch(url);
        if (res.status !== 404 && !isCancelled) {
          setVideoReady(prev => ({ ...prev, [videoID]: true }));
        }
      }
    };

    const intervalId = setInterval(() => {
      checkVideos();
    }, 1000);

    return () => {
      isCancelled = true;
      clearInterval(intervalId);
    };
  }, [videos.data, videoReady]);

  return (
    <>
      <Head>
        <title>Televisions</title>
        <meta name="description" content="why is she taking the picture?" />
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <main>
        <div id="container">
          {videoIDs.map((id, index) => (
            <VideoComponent key={index} video_id={videoReady[id] ? id : null} />
          ))}
          <img id="mask" src="/web/televisions_mask.png"></img>
        </div>
      </main>
    </>
  );
}
