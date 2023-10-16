import OpenAI from 'openai';

import { z } from "zod";

import { createTRPCRouter, publicProcedure } from "~/server/api/trpc";

const openai = new OpenAI({
});

export const openaiRouter = createTRPCRouter({


  get_topic: publicProcedure
    //.input(z.object({ text: z.string() }))
    .query(async () => {

      const prompt = `
      I want you to provide me the name of a natural disaster or war that occurred after 1960. 
      Please provide the name only and no date information. I want only the name in the response.
      I want you to not pick the first thing that comes to mind, but rather a lesser known event. 
      I would like you to pick randomly from your ideas, because I am going to ask you this many, many times and I want as many different answers as possible.
      `

      const params: OpenAI.Chat.ChatCompletionCreateParams = {
        messages: [{ role: 'user', content: prompt }],
        model: 'gpt-4',
        // model: 'gpt-3.5-turbo',
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