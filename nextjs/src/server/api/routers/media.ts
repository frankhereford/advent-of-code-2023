import { z } from "zod";

import { createTRPCRouter, publicProcedure } from "~/server/api/trpc";

import fs from 'fs';

const dirPath = '/application/media/';

const getDirectories = (source: string): string[] => {
  return fs.readdirSync(source, { withFileTypes: true })
    .filter(dirent => dirent.isDirectory())
    .map(dirent => dirent.name);
}

export const mediaRouter = createTRPCRouter({
  top_television: publicProcedure
    .input(z.object({ topic: z.string() }))
    .query(() => {
      const directories: string[] = getDirectories(dirPath);
      const randomIndex: number = Math.floor(Math.random() * directories.length);
      const randomDir: string = directories[randomIndex]!;
      return {
        video_id: randomDir,
      };
    }),
  bottom_television: publicProcedure
    .input(z.object({ topic: z.string() }))
    .query(() => {
      const directories: string[] = getDirectories(dirPath);
      const randomIndex: number = Math.floor(Math.random() * directories.length);
      const randomDir: string = directories[randomIndex]!;
      return {
        video_id: randomDir,
      };
    }),
});
