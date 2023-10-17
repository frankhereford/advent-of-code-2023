import { useEffect, useState } from 'react';
import Head from "next/head";

import VideoComponent from './VideoComponent'; 

import { api } from "~/utils/api";

function getStatic(): string | null {
  // "yCPtemRqJks" vernoi noise, yech
  // "ORhPhhcdtkA" had a message in it, yech
  // "OUoA-dKQhtc" unnatural noise
  const myArray: string[] = [
    "CKsLgdbzlfk", "S7askw5Z084", "bVy32Lx41-o",
    "N0OPOsN3kA8", "htdAiUyoFDU", "nqtwZXp88w0", "vcZv6yfBcLk",
    "mCcNtjVBL3w", "IeRJnQBySCg"];

  if (myArray.length === 0) {
    return null;
  }

  const randomIndex = Math.floor(Math.random() * myArray.length);
  return myArray[randomIndex]!;
}


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

          // <div id='black'></div>
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
            <VideoComponent key={index} video_id={videoReady[id] ? id : getStatic()} />
          ))}
          <img id="mask" src="/web/televisions_mask.png"></img>
        </div>
      </main>
    </>
  );
}
