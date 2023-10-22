import { useEffect, useState } from 'react';
import Head from "next/head";
import { api } from "~/utils/api";

import Polaroid from './components/Polaroid';

export default function Home() {
  const [videoIDs, setVideoIDs] = useState<(string | null)[]>([null, null]);
  const [label, setLabel] = useState<(string | null)>('tuna & fish');

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
          <Polaroid videoIDs={videoIDs} label={label} />
        </div>
      </main>
    </>
  );
}
