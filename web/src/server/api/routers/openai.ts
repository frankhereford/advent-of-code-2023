import OpenAI from 'openai';

import { z } from "zod";

import { createTRPCRouter, publicProcedure } from "~/server/api/trpc";

const openai = new OpenAI({
});

export const openaiRouter = createTRPCRouter({
  hello: publicProcedure
    .input(z.object({ text: z.string() }))
    .query(async ({ input }) => {
      const params: OpenAI.Chat.ChatCompletionCreateParams = {
        messages: [{ role: 'user', content: 'Please provide a single notable war or natural disaster that happened since 1960. please try to pick important but not "top-tier" choices. I will be asking you this question many times, and I want to minimize the amount of repetition I get from you. Please answer in the form of "Name of Event (year or years active)."' }],
        model: 'gpt-3.5-turbo',
      };
      const chatCompletion: OpenAI.Chat.ChatCompletion | undefined = await openai.chat.completions.create(params);
      if (chatCompletion?.choices[0]?.message?.content) {
        console.log(chatCompletion.choices[0].message.content)
        return {
          greeting: `Prompt: ${chatCompletion.choices[0].message.content}`,
        };
      } else {
        return {
          greeting: 'Error: Could not retrieve prompt from OpenAI',
        };
      }
    }),
});
