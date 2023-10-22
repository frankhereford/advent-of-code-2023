import { env } from "../../../env.mjs";
import { z } from "zod";
import { createTRPCRouter, publicProcedure } from "~/server/api/trpc";
import { google, youtube_v3 } from 'googleapis';

import { createClient } from "redis";

const youtube = google.youtube({
  version: 'v3',
  auth: env.YOUTUBE_API_KEY,
});

async function searchYouTube(query: string) {
  console.log('in here')
  try {
    const res = await youtube.search.list({
      part: 'snippet',
      q: query,
      type: 'video',
      maxResults: 10, 
    });

    const videos = res.data.items;

    if (videos) {
      videos.forEach((video: youtube_v3.Schema$SearchResult) => {
        console.log(`Title: ${video.snippet?.title}`);
        console.log(`Description: ${video.snippet?.description}`);
      });
    } else {
      console.log('No videos found.');
    }
  } catch (err) {
    console.error(`An error occurred: ${err}`);
  }
}

function getRandomElements(arr: string[], n: number, neverPicks: string[] = []): string[] {
  const filtered = arr.filter(item => !neverPicks.includes(item));
  const shuffled = [...filtered].sort(() => 0.5 - Math.random());
  return shuffled.slice(0, n);
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
async function getRandomVideoKeysFromRedis(redisClient: any, existingPicks: string[]): Promise<string[]> {
  const keysPattern = "video:*";
  // eslint-disable-next-line @typescript-eslint/no-unsafe-assignment, @typescript-eslint/no-unsafe-member-access, @typescript-eslint/no-unsafe-call
  const allKeys = await redisClient.keys(keysPattern);
  const numToPick = 2 - existingPicks.length; // If existingPicks has 0 or 1 element, pick 2 or 1 more
  // eslint-disable-next-line @typescript-eslint/no-unsafe-argument, @typescript-eslint/no-unsafe-member-access, @typescript-eslint/no-unsafe-call
  return getRandomElements(allKeys.map((key: string) => key.split(":")[1]), numToPick, existingPicks);
}

export const youtubeRouter = createTRPCRouter({
  get_video: publicProcedure
    .input(z.object({ topic: z.string() }))
    .query(async ({ input }) => {
      const redisClient = await createClient({
        url: 'redis://redis'
      }).connect();

      // old api
      //const results = await youtubesearchapi.GetListByKeyword(input.topic, false, 10, [{ type: "video", videoDuration: "short" }]);
      //const videoIds = results.items
        //.filter((item: { type: string; }) => item.type === 'video')
        //.map((item: { id: any; }) => item.id);

      // this is the new api
      //const results = await searchYouTube(input.topic)

      // compute return array here when I can search again


      let picks: string[] = ['eE1RjBJ0wjE'];

      // old
      //picks = getRandomElements(videoIds, 2, ["zQy9sbRuMUw"]);

      // Adding from Redis, if needed
      const additionalPicks = await getRandomVideoKeysFromRedis(redisClient, picks);
      picks = [...picks, ...additionalPicks];

      console.log(picks);

      if (picks.length > 0) {
        await redisClient.rPush('start_queue', picks);
      }
      else {
        console.log("No videos found");
      }

      return {
        videos: picks
      };
    }),
});