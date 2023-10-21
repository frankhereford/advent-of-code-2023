/* eslint-disable @typescript-eslint/no-unsafe-return */
/* eslint-disable @typescript-eslint/no-unsafe-argument */
/* eslint-disable @typescript-eslint/no-unsafe-call */
/* eslint-disable @typescript-eslint/no-unsafe-member-access */
/* eslint-disable @typescript-eslint/no-unsafe-assignment */
/* eslint-disable @typescript-eslint/no-explicit-any */
import { env } from "../../../env.mjs";
import { z } from "zod";
import { createTRPCRouter, publicProcedure } from "~/server/api/trpc";
import youtubesearchapi from "youtube-search-api";

import { createClient } from "redis";


function getRandomElements(arr: string[], n: number, neverPicks: string[] = []): string[] {
  const filtered = arr.filter(item => !neverPicks.includes(item));
  const shuffled = [...filtered].sort(() => 0.5 - Math.random());
  return shuffled.slice(0, n);
}

async function getRandomVideoKeysFromRedis(redisClient: any, existingPicks: string[]): Promise<string[]> {
  const keysPattern = "video:*";
  const allKeys = await redisClient.keys(keysPattern);
  const numToPick = 2 - existingPicks.length; // If existingPicks has 0 or 1 element, pick 2 or 1 more
  return getRandomElements(allKeys.map((key: string) => key.split(":")[1]), numToPick, existingPicks);
}

export const youtubeRouter = createTRPCRouter({
  get_video: publicProcedure
    .input(z.object({ topic: z.string() }))
    .query(async ({ input }) => {
      const results = await youtubesearchapi.GetListByKeyword(input.topic, false, 10, [{ type: "video", videoDuration: "short" }]);
      const videoIds = results.items
        .filter((item: { type: string; }) => item.type === 'video')
        .map((item: { id: any; }) => item.id);

      const redisClient = await createClient({
        url: 'redis://redis'
      }).connect();

      let picks: string[] = [];

      picks = getRandomElements(videoIds, 2, ["zQy9sbRuMUw"]);

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