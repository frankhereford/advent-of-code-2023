import fs from 'fs';
import path from 'path';
import { z } from "zod";
import { createTRPCRouter, publicProcedure } from "~/server/api/trpc";
import videos_of_static from "../../../utils/videos_of_static"

const hlsPath = '/application/media/hls/';

const getVideos = (srcPath: string, num: number): string[] => {
  const allDirs = fs.readdirSync(srcPath)
    .filter(file => fs.statSync(path.join(srcPath, file)).isDirectory())
    .filter(dir => !videos_of_static.includes(dir));

  // Fisher-Yates Shuffle
  for (let i = allDirs.length - 1; i > 0; i--) {
    const j = Math.floor(Math.random() * (i + 1));
    [allDirs[i], allDirs[j]] = [allDirs[j]!, allDirs[i]!];
  }

  return allDirs.slice(0, num);
};

export const mediaRouter = createTRPCRouter({
  get_random_videos: publicProcedure
    .input(z.object({ length: z.number() }))
    .query(({ input }) => {
      const directories = getVideos(hlsPath, input.length);
      return directories;
    }),
});
