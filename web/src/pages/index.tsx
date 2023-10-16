import Head from "next/head";

import VideoComponent from './VideoComponent'; 

import { api } from "~/utils/api";

export default function Home() {
  const topic = api.openai.get_topic.useQuery();
  const videos = api.youtube.get_video.useQuery({topic: 'parks'}, { enabled: !!topic.data });
  const top_video = api.video.top_television.useQuery({topic: 'tuba'}, {enabled: !!videos.data});
  const bottom_video = api.video.bottom_television.useQuery({ topic: 'fish' }, { enabled: !!videos.data});

  return (
    <>
      <Head>
        <title>Televisions</title>
        <meta name="description" content="why is she taking the picture?" />
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <main>
        <div id="container">
          <VideoComponent video_id={top_video.data ? top_video.data.video_id : null } />
          <VideoComponent video_id={bottom_video.data ? bottom_video.data.video_id : null } />
          <img id="mask" src="/web/televisions_mask.png"></img>
        </div>
      </main>
    </>
  );
}
