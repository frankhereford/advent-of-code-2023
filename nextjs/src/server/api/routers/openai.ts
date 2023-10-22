import OpenAI from 'openai';

import { z } from "zod";

import { createTRPCRouter, publicProcedure } from "~/server/api/trpc";

const openai = new OpenAI({
});

const getRandomElement = (arr: string[]): string => {
  const randomIndex = Math.floor(Math.random() * arr.length);
  return arr[randomIndex]!;
};

export const openaiRouter = createTRPCRouter({
  get_topic: publicProcedure
    .input(z.object({ hint: z.string() }))
    .query(async () => {

      const prompts: string[] = [`
      I want you to take whatever input I provide and think about what major 
      news event has happened since 1970 that is most associated to what I say. 
      When you pick, I want you to come up with a list of 10 things that could
      be good answers, and I want you to randomly select your choice. I don't 
      want to get the same answer every time. I want you to respond with the 
      name of that event, as it would be used to search for a video on youtube.
      I want you to respond with just an appropriate youtube search query, 
      and no additional text. If this is your first response, proceed without
      any input.
      `]

      // prompts = prompts.filter(item => item.includes('musicians'));


      const prompt = getRandomElement(prompts)

      const params: OpenAI.Chat.ChatCompletionCreateParams = {
        messages: [{ role: 'user', content: prompt }],
        model: 'gpt-4',
        //model: 'gpt-3.5-turbo',
      };
      const chatCompletion: OpenAI.Chat.ChatCompletion | undefined = await openai.chat.completions.create(params);
      if (chatCompletion?.choices[0]?.message?.content) {
        //console.log(chatCompletion.choices[0].message.content)
        return {
          topic: `${chatCompletion.choices[0].message.content}`,
        };
      } else {
        return {
          topic: 'Error: Could not retrieve prompt from OpenAI',
        };
      }
    }),
});