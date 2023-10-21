import { useEffect, useState } from 'react';
import Head from "next/head";
import Image from "next/image";
import { api } from "~/utils/api";

import VideoComponent from './VideoComponent'; 

export default function Home() {
  const [videoIDs, setVideoIDs] = useState<(string | null)[]>([null, null]);

  const videos = api.media.get_random_videos.useQuery({ length: 2 })
  const topic = api.openai.get_topic.useQuery();
  const youtube = api.youtube.get_video.useQuery({ topic: topic.data?.topic }, { enabled: !!topic.data });

  useEffect(() => {
    if (!videos.data) return;
    setVideoIDs(videos.data);
  }, [videos.data]);

  return (
    <>
      <Head>
        <title>Televisions</title>
        <meta name="description" content="why is she taking the picture?" />
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <main>
        <div className="container">
          <div className="photograph">
            {videoIDs.map((id, index) => (
              <VideoComponent key={index} video_id={id} />
            ))}
            <Image
              alt='televisions in the window of a repair shop'
              src="/televisions_mask.png"
              width={1024}
              height={1024}
            />
          </div>
        </div>
      </main>
    </>
  );
}
