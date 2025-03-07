import "dotenv/config";
import { ansiColorFormatter } from "@logtape/logtape";
import { drizzle } from "drizzle-orm/node-postgres";
import { Elysia } from "elysia";
import taskRoute from "./routes/tasks";
import { configure, getConsoleSink, getLogger } from "@logtape/logtape";

await configure({
  sinks: {
    console: getConsoleSink({
      formatter: ansiColorFormatter,
    }),
  },
  loggers: [
    { category: "hivemind", lowestLevel: "debug", sinks: ["console"] },
    {
      category: ["logtape", "meta"],
      lowestLevel: "warning",
      sinks: ["console"],
    },
  ],
});
const l = getLogger(["hivemind", "startup"]);

export const db = drizzle(process.env.DATABASE_URL!);

const port = 7737;
l.info("starting server...");
l.info(`listening at port ${port}`);

new Elysia({ normalize: false }).use(taskRoute).listen(7737);
