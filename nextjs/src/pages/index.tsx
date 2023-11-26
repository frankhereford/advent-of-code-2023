import { useEffect, useState } from 'react';
import Head from "next/head";
import { api } from "~/utils/api";

import Polaroid from '~/pages/components/Polaroid';
import Rust from '~/pages/components/Rust';
import Terminal from '~/pages/components/Terminal';

export default function Home() {
  const [videoIDs, setVideoIDs] = useState<(string | undefined)[]>(Array.from({ length: 2 }, () => undefined));
  const [label, setLabel] = useState<(string | undefined)>('');
  const [rustUpdates, setRustUpdates] = useState(''); // State variable for updates from Rust component


  const videos = api.television.think.useQuery({ user_input: '' }, {
    trpc: { context: { skipBatch: true, }, },
    refetchOnWindowFocus: false,
  });

  useEffect(() => {
    if (videos.data) {
      setVideoIDs(videos.data.videos);
      setLabel(videos.data.label);
    }
  }, [videos.data]);

  useEffect(() => {
    console.log(videos.data)
    if (videos.data) {
      setVideoIDs(videos.data.videos);
    }
  }, [videos.data]);

  const handleRustUpdate = (update: string) => {
    setRustUpdates(prev => prev + update); // Accumulate updates
    console.log("tacoland:", update)
  };


  return (
    <>
      <Head>
        <title>Televisions</title>
        <meta name="description" content="why is she taking the picture?" />
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <main>
        <Rust onUpdate={handleRustUpdate}></Rust>
        <Terminal content={rustUpdates} speed={.8} variability={3} />
        <div className="container">
          <Polaroid videoIDs={videoIDs} label={label} />
        </div>
      </main>
    </>
  );
}
