import axios from 'axios';
import { env } from "../../../env.mjs";

import { z } from "zod";

import { createTRPCRouter, publicProcedure } from "~/server/api/trpc";

async function fetchYouTubeVideoId(searchQuery: string): Promise<string | null> {
  console.log('hi')
  try {
    const apiKey = env.YOUTUBE_API_KEY
    const url = 'https://www.googleapis.com/youtube/v3/search';
    const params = {
      part: 'snippet',
      maxResults: 1,
      q: searchQuery,
      key: apiKey,
    };

    const response = await axios.get(url, { params });

    console.log(response)

    if (response.data.items && response.data.items.length > 0) {
      return response.data.items[0].id.videoId;
    }

    return null;
  } catch (error) {
    //console.error('Something went wrong:', error);
    return null;
  }
}

export const youtubeRouter = createTRPCRouter({

  get_video: publicProcedure
    .input(z.object({ topic: z.string() }))
    .query(async ({input}) => {

      const video_id = await fetchYouTubeVideoId(input.topic)

      //console.log(video_id)

      return {
      video_id: video_id
    }

    }),
});