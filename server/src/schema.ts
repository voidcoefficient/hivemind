import { relations } from "drizzle-orm";
import {
  boolean,
  integer,
  pgTable,
  primaryKey,
  timestamp,
  varchar,
} from "drizzle-orm/pg-core";

export const tasks = pgTable("tasks", {
  id: integer().primaryKey().generatedAlwaysAsIdentity(),
  title: varchar({ length: 255 }).notNull(),
  description: varchar({ length: 255 }),
  completed: boolean().default(false),
  createdAt: timestamp("created_at").defaultNow(),
  updatedAt: timestamp("updated_at").defaultNow(),
});

export const assets = pgTable("assets", {
  id: integer().primaryKey().generatedAlwaysAsIdentity(),
  title: varchar({ length: 255 }).notNull(),
  description: varchar({ length: 255 }),
  amount: integer().default(0).notNull(),
  createdAt: timestamp("created_at").defaultNow(),
  updatedAt: timestamp("updated_at").defaultNow(),
});

export const tags = pgTable("tags", {
  id: integer().primaryKey().generatedAlwaysAsIdentity(),
  title: varchar({ length: 255 }).notNull(),
  description: varchar({ length: 255 }),
  createdAt: timestamp("created_at").defaultNow(),
  updatedAt: timestamp("updated_at").defaultNow(),
});

export const taskTags = pgTable(
  "task_tags",
  {
    taskId: integer("task_id")
      .notNull()
      .references(() => tasks.id),
    tagId: integer("tag_id")
      .notNull()
      .references(() => tags.id),
  },
  (t) => [
    primaryKey({
      columns: [t.taskId, t.tagId],
    }),
  ],
);

export const taskRelations = relations(tasks, ({ many }) => ({
  tags: many(tags),
}));

export const tagRelations = relations(tags, ({ many }) => ({
  tasks: many(tasks),
}));

export const taskTagsRelations = relations(taskTags, ({ one }) => ({
  task: one(tasks, {
    fields: [taskTags.taskId],
    references: [tasks.id],
  }),
  tag: one(tags, {
    fields: [taskTags.tagId],
    references: [tags.id],
  }),
}));
