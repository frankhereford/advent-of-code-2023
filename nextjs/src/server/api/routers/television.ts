import OpenAI from 'openai';
import { z } from "zod";

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

export const televisionRouter = createTRPCRouter({
  think: publicProcedure
    .input(z.object({ user_input: z.string() }))
    .query(async ({ input }) => {
      const subject = await getOpenAIResponse(input.user_input);
      console.log(subject)
    }),
});