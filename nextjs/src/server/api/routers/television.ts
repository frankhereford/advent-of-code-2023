/* eslint-disable @typescript-eslint/no-unsafe-member-access */
import OpenAI from 'openai';
import { google } from 'googleapis';
import { createClient } from "redis";
import { z } from "zod";
import * as fs from 'fs';
import * as path from 'path';

import { env } from "../../../env.mjs";
import { createTRPCRouter, publicProcedure } from "~/server/api/trpc";

const openai = new OpenAI({
});

function getRandomYear(): number {
  const minYear = 1970;
  const maxYear = 2023;
  return Math.floor(Math.random() * (maxYear - minYear + 1)) + minYear;
}
const getRandomElement = (arr: string[]): string => {
  const randomIndex = Math.floor(Math.random() * arr.length);
  return arr[randomIndex]!;
};

interface OpenAiResponseType {
  label: string;
  topic: string;
}

const getOpenAIResponse = async (input: string): Promise<OpenAiResponseType> => {
  let hint = input;
  if (input === '') hint = getRandomYear().toString();

  const prompts: string[] = [`
      All responses must come in the form of a JSON object with two keys.
      The first key is named 'label' and the second key is named 'topic'. I want you
      to only respond with the JSON object and absolutely nothing else. I am going to 
      parse your response as JSON. Here are the instructions for generating the 
      values for each key.

      I want you to think of a major news story since 1970 that is most associated
      to this hint: '${hint}'.
      
      When you pick the topic, I want you to come up with a list of 10 things that could
      be good answers, and I want you to randomly select your choice. I don't 
      want to get the same answer every time. Really try to pick "B-list" news events.
      Don't be obvious.
      
      For the 'label' key, I want you to respond with the name of that event, 
      however, I want you to be very brief. Four words or fewer. Please prefer lowercase 
      unless it would read poorly without capitalization.  I am going to use this
      as a label on an image of polaroid photo.

      For the 'topic' key, I want you to respond with whatever string you think
      would be a good search query for youtube to find news footage for the news
      event that you picked.
      `]

  const prompt = getRandomElement(prompts)

  const params: OpenAI.Chat.ChatCompletionCreateParams = {
    messages: [{ role: 'user', content: prompt }],
    model: 'gpt-4',
    //model: 'gpt-3.5-turbo', // too hard of a prompt for 3.5 generally
  };

  const chatCompletion: OpenAI.Chat.ChatCompletion | undefined = await openai.chat.completions.create(params);

  // this feels very risky. this should trap gpt 4 giving bogus JSON and try again.
  const openAiResponse = JSON.parse(chatCompletion.choices[0]!.message.content!) as OpenAiResponseType;
  return openAiResponse;
}


const getYouTubeVideos = async (topic: string): Promise<string[]> => {
  // Initialize the YouTube API client
  const youtube = google.youtube({
    version: "v3",
    auth: env.YOUTUBE_API_KEY
  });

  const params = {
    q: topic,
    part: "snippet",
    type: "video",
    maxResults: 5,
  };

  // Perform the search
  async function searchYouTube() {
      const response = await youtube.search.list(params);
      // eslint-disable-next-line @typescript-eslint/no-unsafe-assignment
      const results = response.data.items;
      // eslint-disable-next-line @typescript-eslint/no-unsafe-return
      return results
    }

  // Execute the function
  // eslint-disable-next-line @typescript-eslint/no-unsafe-assignment
  const results = await searchYouTube();

  type YoutubeSearchResult = {
    kind: string;
    etag: string;
    id: { kind: string; videoId: string };
    snippet: {
      publishedAt: string;
      channelId: string;
      title: string;
      description: string;
      thumbnails: object;
      channelTitle: string;
      liveBroadcastContent: string;
      publishTime: string;
    };
  };

  const getRandomVideoIds = (data: YoutubeSearchResult[], n: number): string[] => {
    // Shuffle the original array
    const shuffledData = [...data].sort(() => Math.random() - 0.5);
    // Get first n items
    const selectedItems = shuffledData.slice(0, n);
    // Extract videoIds
    const videoIds = selectedItems.map(item => item.id.videoId);
    return videoIds;
  };

  // FIXME this 2 is magic. it should be passed down.
  const randomVideoIds = getRandomVideoIds(results, 2);
  return randomVideoIds;
}

// Function to add array elements to 'start_encode' Redis list
const addToRedisQueue = async (videos: string[]): Promise<void> => {
  const redisClient = await createClient({
    url: 'redis://redis'
  }).connect();

  if (videos.length > 0) {
    await redisClient.rPush('start_queue', videos);
  }
  else {
    console.log("No videos found");
  }
}

async function checkAllFilesExist(videoIds: string[]): Promise<boolean> {
  let allExist = false;

  while (!allExist) {
    allExist = true; // Reset for each iteration

    for (const videoId of videoIds) {
      const m3u8Path = path.join("/application/media/hls", videoId, "playlist.m3u8");
      const tsPath = path.join("/application/media/hls", videoId, "stream001.ts");

      try {
        await fs.access(m3u8Path);
        await fs.access(tsPath);
      } catch (err) {
        allExist = false;
        break; // Exit the loop early if any file doesn't exist
      }
    }

    if (!allExist) {
      await new Promise(resolve => setTimeout(resolve, 1000)); // Wait 1 second before next iteration
    }
  }

  return true;
}


export const televisionRouter = createTRPCRouter({
  think: publicProcedure
    .input(z.object({ user_input: z.string() }))
    .query(async ({ input }) => {
      const subject = await getOpenAIResponse(input.user_input);
      console.log(subject)
      const videos = await getYouTubeVideos(subject.topic);
      console.log(videos)
      await addToRedisQueue(videos)
      console.log("waiting for files to exist")
      await checkAllFilesExist(videos);
      console.log("files exist")
      return { videos: videos };
    }),
});