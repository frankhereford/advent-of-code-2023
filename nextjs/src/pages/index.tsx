import { useEffect, useState } from 'react';
import Head from "next/head";
import Image from "next/image";
import { api } from "~/utils/api";

import Video from './components/Video'; 

export default function Home() {
  const [videoIDs, setVideoIDs] = useState<(string | null)[]>([null, null]);

  const videos = api.media.get_random_videos.useQuery({ length: 2 })

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
          <div className='polaroid'>
            <div className="photograph">
              {videoIDs.map((id, index) => (
                <Video key={index} video_id={id} />
              ))}
              <Image
                alt='televisions in the window of a repair shop'
                src="/televisions_mask.png"
                width={1024}
                height={1024}
              />
            </div>
            <div className='caption'>
              World Trade Center Bombing
            </div>
          </div>
        </div>
      </main>
    </>
  );
}
