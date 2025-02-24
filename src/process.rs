use std::env;

use anyhow::Result;
use async_nats::Client;
use futures::StreamExt;
use sqlx::{PgPool, query};
use tracing::info;

use crate::tasks::{CreateTask, CreateTaskSerde, Model};

pub const TASK_SUBJECT: &str = "tasks";
pub const TASK_CREATE_QUEUE: &str = "queues.tasks.create";

pub struct TaskProcessor {
	client: Client,
}

impl TaskProcessor {
	pub fn new(client: Client) -> Self {
		Self { client }
	}

	async fn insert(pool: &PgPool, task: CreateTask) -> Result<String> {
		let rec = query!(
			r#"insert into tasks (id, title, description, completed) values ($1, $2, $3, $4) returning id"#,
			uuid::Uuid::new_v4(),
			task.title,
			task.description,
			task.completed
		)
		.fetch_one(pool)
		.await?;

		Ok(rec.id.to_string())
	}

	pub async fn process_tasks(&self) -> Result<()> {
		let pool = PgPool::connect(&env::var("DATABASE_URL")?).await?;
		let subscription = self
			.client
			.queue_subscribe(TASK_SUBJECT, TASK_CREATE_QUEUE.to_owned())
			.await?;

		subscription
			.for_each(async |message| {
				if let Some(task) = CreateTaskSerde::deserialize(message.payload) {
					let task_id = TaskProcessor::insert(&pool, task).await.unwrap();
					dbg!(task_id.clone());
					info!("processed task \"{}\" into postgres", task_id);
				}
			})
			.await;

		Ok(())
	}
}
