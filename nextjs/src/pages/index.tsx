import { useEffect, useState } from 'react';
import Head from "next/head";

import VideoComponent from './VideoComponent'; 

export default function Home() {
  const [videoIDs, setVideoIDs] = useState<(string | null)[]>([null, null]);

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
            <VideoComponent key={index} video_id={id} />
          ))}
          <img id="mask" alt='televisions in the window of a repair shop' src="/televisions_mask.png"></img>
        </div>
      </main>
    </>
  );
}
