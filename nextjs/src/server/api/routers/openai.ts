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
    //.input(z.object({ text: z.string() }))
    .query(async () => {

      const prompts: string[] = [`
      I want you to provide me the name of a natural disaster or war that occurred after 1960. 
      Please provide the name only and no date information. I want only the name in the response.
      I want you to not pick the first thing that comes to mind, but rather a lesser known event. 
      I would like you to pick randomly from your ideas, because I am going to ask you this 
      many, many times and I want as many different answers as possible.
      `, `
      I want you to think of a media property from the 70s, 80s, or 90s.
      Please say the name of it in this format: <Media Name>
      I want you to pick something sort of "B-list". Don't pick the obvious stuff.
      Don't say anything else.
      Don't pick fucking quantum leap anymore.
      `, `
      say "bill wurtz" and nothing else
      `, `
      Give me the media name for a interesting thing that happened to a US president."
      I want you to pick something sort of "B-list". Don't pick the obvious stuff.
      Don't say anything else.
      `, `
      Pick a famous scientific discovery from the last century. 
      Tell me the name of the discovery plainly.
      Just the name. Like "Discovering radioactivity."
      Don't say anything else.
      `, `
      Give me a brief title of a subject that makes Texas uncomfortable.
      Only pick gun control about 1/4 of the time instead of 100% of the time.
      `, `
      Give me the first and last name of a famous person who acted in movies between 1950 and 1999.
      Just the first and last name please.
      Please pick someone who is B-list, and pick them very randomly.
      `, `
      Give me the name of a Criterion Collection Movie.
      Come up with a list of a lot of them and then pick one randomly. Give me just the name
      Pick a B-list choice. Don't pick the obvious stuff.
      `, `
      Give me the name of a huricane and the year it occured.
      Pick a B-list choice. Don't pick the obvious stuff.
      `, `
      Give me the name of a US city.
      It doesn't need to be big. Maybe big enough to have a youtube video about it.
      Pick a B-list choice. Don't pick the obvious stuff.
      `, `
      Really cool musicians who were cool as fuck. Only ones who had a lot of videos.
      `, `
      Tell me the name of a saturday morning cartoon. Just that. Just the name.
      `]

      // prompts = prompts.filter(item => item.includes('musicians'));


      const prompt = getRandomElement(prompts)

      const params: OpenAI.Chat.ChatCompletionCreateParams = {
        messages: [{ role: 'user', content: prompt }],
        //model: 'gpt-4',
        model: 'gpt-3.5-turbo',
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