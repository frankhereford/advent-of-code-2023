import { useEffect, useState } from 'react';
import Head from "next/head";
import { api } from "~/utils/api";
import { useRouter } from 'next/router'
import z from 'zod'

import Polaroid from '~/pages/components/Polaroid';
import Rust from '~/pages/components/Rust';
import Terminal from '~/pages/components/Terminal';
import GitHub from '~/pages/components/GitHub';
import packageJson from '~/../package.json' assert { type: 'json' }


export const defaultDay = packageJson.defaultDay as number;

export default function Home() {
  const [day, setDay] = useState(3)
  const router = useRouter()
  const [videoIDs, setVideoIDs] = useState<(string | undefined)[]>(Array.from({ length: 2 }, () => undefined));
  const [label, setLabel] = useState<(string | undefined)>('');
  const [rustUpdates, setRustUpdates] = useState(''); // State variable for updates from Rust component


  useEffect(() => {
    if (!router.isReady) return
    if (typeof router.query.index !== 'string') return
    const daySchema = z.number()
    const parseResult = daySchema.safeParse(parseInt(router.query.index))
    if (parseResult.success && parseResult.data >= 1 && parseResult.data <= defaultDay) {
      setDay(parseResult.data)
    } else {
      setDay(defaultDay)
    }
  }, [router.isReady])


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
    //console.log(videos.data)
    if (videos.data) {
      setVideoIDs(videos.data.videos);
    }
  }, [videos.data]);

  const handleRustUpdate = (update: string) => {
    setRustUpdates(prev => prev + update); // Accumulate updates
  };


  return (
    <>
      <Head>
        <title>Televisions</title>
        <meta name="description" content="why is she taking the picture?" />
        <link rel="icon" href="data:image/svg+xml,<svg xmlns=%22http://www.w3.org/2000/svg%22 viewBox=%220 0 100 100%22><text y=%22.9em%22 font-size=%2290%22>📺</text></svg>" />
      </Head>
      <main>
        <GitHub />
        <Rust day={day} onUpdate={handleRustUpdate}></Rust>
        <Terminal content={rustUpdates} speed={.2} variability={3} />
        <div className="container">
          <Polaroid videoIDs={videoIDs} label={label} />
        </div>
      </main>
    </>
  );
}
