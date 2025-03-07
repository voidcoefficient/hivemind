import { createInsertSchema, createUpdateSchema } from "drizzle-typebox";
import { Elysia, t } from "elysia";
import { tasks } from "../schema";
import { eq } from "drizzle-orm";
import { db } from "..";
import { getLogger } from "@logtape/logtape";

const l = getLogger(["hivemind", "tasks"]);

const taskInsertSchema = createInsertSchema(tasks);
const taskUpdateSchema = createUpdateSchema(tasks);

export default new Elysia({
  normalize: false,
}).group("/tasks", (app) =>
  app
    .get("/", async () => {
      l.debug("returning task list");
      return await db.select().from(tasks);
    })
    .get(
      "/:id",
      async ({ params }) => {
        l.debug("returning task");
        return await db.select().from(tasks).where(eq(tasks.id, params.id));
      },
      {
        params: t.Object({
          id: t.Integer(),
        }),
      },
    )
    .post(
      "/",
      async ({ body }) => {
        l.debug("inserting task");
        await db.insert(tasks).values(body);
        l.debug("task inserted successfully");

        return { message: "added task successfully" };
      },
      {
        body: taskInsertSchema,
      },
    )
    .patch(
      "/:id",
      async ({ body, params }) => {
        l.debug("updating task");
        await db.update(tasks).set(body).where(eq(tasks.id, params.id));
        l.debug("task updated successfully");

        return { message: "updated task successfully" };
      },
      {
        body: taskUpdateSchema,
        params: t.Object({
          id: t.Numeric(),
        }),
      },
    )
    .delete(
      "/:id",
      async ({ params }) => {
        l.debug("deleting task");
        await db.delete(tasks).where(eq(tasks.id, params.id));
        l.debug("task deleted successfully");
      },
      {
        params: t.Object({
          id: t.Numeric(),
        }),
      },
    ),
);
