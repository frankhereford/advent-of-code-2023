import { useRef, useEffect } from "react";
import Head from "next/head";


import videojs from "video.js";
import "video.js/dist/video-js.css";

import { api } from "~/utils/api";


export default function Home() {
  const hello = api.example.hello.useQuery({ text: "from tRPC" });

  /*
  const videoRef = useRef(null);

  useEffect(() => {
    if (videoRef.current) {
      videojs(videoRef.current, {
        sources: [
          {
            src: "/video_processor/X-iSQQgOd1A/playlist.m3u8",
            type: "application/x-mpegURL"
          }
        ]
      });
    }
  });
  */

          // <video id="top_television" controls autoPlay className="video-js" />

  return (
    <>
      <Head>
        <title>Televisions</title>
        <meta name="description" content="Generated by create-t3-app" />
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <main>
        <div id="container">
          <img id="mask" src="/web/televisions_mask.png"></img>
        </div>
      </main>
    </>
  );
}
