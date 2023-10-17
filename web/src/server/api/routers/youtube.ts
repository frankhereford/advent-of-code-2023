import { env } from "../../../env.mjs";
import { z } from "zod";
import { createTRPCRouter, publicProcedure } from "~/server/api/trpc";
const youtubesearchapi = require("youtube-search-api");

import { createClient } from "redis";


function getRandomElements(arr: string[], n: number, neverPicks: string[] = []): string[] {
  const filtered = arr.filter(item => !neverPicks.includes(item));
  const shuffled = [...filtered].sort(() => 0.5 - Math.random());
  return shuffled.slice(0, n);
}

export const youtubeRouter = createTRPCRouter({

  get_video: publicProcedure
    .input(z.object({ topic: z.string() }))
    .query(async ({input}) => {

      const results = await youtubesearchapi.GetListByKeyword(input.topic, false, 10, [{ type: "video", videoDuration: "short" }])
      const videoIds = results.items
        .filter((item: { type: string; }) => item.type === 'video')
        .map((item: { id: any; }) => item.id);
      const picks = getRandomElements(videoIds, 2, ["zQy9sbRuMUw"])
      console.log(picks)

      let redisClient = await createClient({
        url: 'redis://redis'
      }).connect();

      await redisClient.rPush('start_queue', picks);

      return {
        videos: picks
    }

    }),
});