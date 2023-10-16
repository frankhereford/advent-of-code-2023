import { useEffect, useState } from 'react';
import Head from "next/head";

import VideoComponent from './VideoComponent'; 

import { api } from "~/utils/api";
export default function Home() {
  
  const [videoReady, setVideoReady] = useState(false);
  const [videoID, setVideoID] = useState('');


  const topic = api.openai.get_topic.useQuery();
  const videos = api.youtube.get_video.useQuery({topic: topic.data!?.topic}, { enabled: !!topic.data });
  
  useEffect(() => {
    if (!videos.data) return;

    let isCancelled = false; // To cleanup if component unmounts
    console.log(videos.data.videos)
    const videoID = videos.data.videos[0]
    console.log(videoID)

    if (!videoID) return;

    const checkVideo = async () => {
      const url = `/video_processor/${videoID}/playlist.m3u8`; // Replace with actual URL
      const res = await fetch(url);
      if (res.status !== 404) {
        if (!isCancelled) {
          setVideoReady(true);
          setVideoID(videoID);
          clearInterval(intervalId);
        }
      }
    };

    const intervalId = setInterval(() => {
      checkVideo();
    }, 1000);

    return () => {
      isCancelled = true;
      clearInterval(intervalId);
    };
  }, [videos.data]);



  //const bottom_video = api.video.bottom_television.useQuery({ topic: 'fish' }, { enabled: !!videos.data});
          //<VideoComponent video_id={top_video.data ? top_video.data.video_id : null } />
          //<VideoComponent video_id={bottom_video.data ? bottom_video.data.video_id : null } />

  return (
    <>
      <Head>
        <title>Televisions</title>
        <meta name="description" content="why is she taking the picture?" />
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <main>
        <div id="container">
          <VideoComponent video_id={videoReady ? videoID : null} />
          <img id="mask" src="/web/televisions_mask.png"></img>
        </div>
      </main>
    </>
  );
}
