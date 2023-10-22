import { use, useEffect, useState } from 'react';
import Head from "next/head";
import { api } from "~/utils/api";

import Polaroid from '~/pages/components/Polaroid';

export default function Home() {
  const [videoIDs, setVideoIDs] = useState<(string | null)[]>(Array.from({ length: 2 }, () => null));
  const [label, setLabel] = useState<(string | undefined)>('');
  const [topic, setTopic] = useState<(string | undefined)>('');
  const [searchYouTube, setSearchYoutube] = useState<(boolean)>(false);

  const topicQuery = api.openai.get_topic.useQuery({ hint: '' }, {
    trpc: { context: { skipBatch: true, }, },
  });

  const videoQuery = api.youtube.get_video.useQuery({ topic: topic! }, {
    enabled: searchYouTube,
    //enabled: false,
    trpc: { context: { skipBatch: true, }, },
  });

  useEffect(() => {
    if (topicQuery.data?.label === '') return;
    if (topicQuery.data) {
      setLabel(topicQuery.data.label)
      setTopic(topicQuery.data.topic)
      setSearchYoutube(true)
    }
  }, [topicQuery.data]);

  useEffect(() => {
    if (!videoQuery.data) return;
    if (videoQuery.data) {
      console.log(videoQuery.data.videos)
      setVideoIDs(videoQuery.data.videos)
    }
  }, [videoQuery.data]);

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
