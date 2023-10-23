import { useEffect, useState } from 'react';
import Head from "next/head";
import { api } from "~/utils/api";

import Polaroid from '~/pages/components/Polaroid';

export default function Home() {
  const [videoIDs, setVideoIDs] = useState<(string | undefined)[]>(Array.from({ length: 2 }, () => undefined));
  const [label, setLabel] = useState<(string | undefined)>('');
  const [topic, setTopic] = useState<(string | undefined)>('');

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
