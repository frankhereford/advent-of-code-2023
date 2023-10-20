import { useEffect, useState } from 'react';
import Head from "next/head";
import Image from "next/image";

import VideoComponent from './VideoComponent'; 

export default function Home() {
  //const [videoIDs, setVideoIDs] = useState<(string | null)[]>([null, null]);

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
            <div className="text">
              Why is she taking the picture?
            </div>
            <Image
              id="mask"
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

          /*
          {videoIDs.map((id, index) => (
            <VideoComponent key={index} video_id={id} />
          ))}
          */