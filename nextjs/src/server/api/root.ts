//import { exampleRouter } from "~/server/api/routers/example";
import { mediaRouter } from "~/server/api/routers/media";
import { openaiRouter } from "~/server/api/routers/openai";
import { youtubeRouter } from "~/server/api/routers/youtube";
import { televisionRouter } from "~/server/api/routers/television";
import { createTRPCRouter } from "~/server/api/trpc";

/**
 * This is the primary router for your server.
 *
 * All routers added in /api/routers should be manually added here.
 */
export const appRouter = createTRPCRouter({
  media: mediaRouter,
  openai: openaiRouter,
  youtube: youtubeRouter,
  television: televisionRouter,
});

// export type definition of API
export type AppRouter = typeof appRouter;
