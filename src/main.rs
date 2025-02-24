pub mod avro;
pub mod nats;
pub mod tasks;

use std::env;

use apache_avro::{Reader, Schema, from_value};
use async_nats::{
	connect,
	jetstream::{self, consumer, stream::DiscardPolicy},
};
use chrono::Local;
use clap::{ArgAction, arg, command, crate_authors};
use futures::StreamExt;
use nats::get_client;
use tasks::{Task, serialize_task};

pub const TASK_RAW_SCHEMA: &str = r#"
  {
    "type": "record",
    "name": "task",
    "fields": [
      { "name": "id", "type": "string", "logicalType": "uuid" },
      { "name": "title", "type": "string" },
      { "name": "description", "type": ["null", "string"], "default": null },
      { "name": "completed", "type": "boolean", "default": false },
      { "name": "created_at", "type": "long", "logicalType": "local-timestamp-millis" },
      { "name": "updated_at", "type": "long", "logicalType": "local-timestamp-millis" },
      { "name": "completed_at", "type": ["null", "long"], "logicalType": "local-timestamp-millis", "default": null }
    ]
  }
"#;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let matches = command!()
		.author(crate_authors!("\n"))
		.subcommand_required(true)
		.subcommand(command!("start").about("starts up hivemind"))
		.subcommand(
			command!("tasks")
				.about("manage tasks")
				.subcommand_required(true)
				.subcommand(
					command!("create")
						.about("create a task")
						.arg(arg!(-t --title <TITLE> "tasks's title").required(true))
						.arg(arg!(-d --description [DESCRIPTION] "tasks's description"))
						.arg(
							arg!(-c --completed [COMPLETED] "if the task is completed or not (default: false)")
								.action(ArgAction::SetTrue),
						),
				)
				.subcommand(command!("edit").about("edit a task"))
				.subcommand(command!("delete").about("delete a task"))
				.subcommand(command!("view").about("view one or more tasks")),
		)
		.get_matches();

	match matches.subcommand() {
		Some(subcommand) => match subcommand {
			("start", _sub_matches) => start_hivemind().await?,
			("tasks", sub_matches) => match sub_matches.subcommand() {
				Some(("create", sub_matches)) => {
					let title = sub_matches.get_one::<String>("title").unwrap().to_owned();
					let description = sub_matches
						.get_one::<String>("description")
						.map(|d| d.to_owned());
					let completed = sub_matches.get_flag("completed");

					let task = Task {
						id: uuid::Uuid::new_v4(),
						title,
						description,
						completed,
						created_at: Local::now().timestamp_millis(),
						updated_at: Local::now().timestamp_millis(),
						completed_at: None,
					};
					let encoded_task = serialize_task(task.clone())?;

					let client = get_client().await?;
					client.publish("tasks", encoded_task).await?;
					client.flush().await?;

					println!("created task \"{}\"", task.id);
				}
				_ => unreachable!("subcommands were covered"),
			},
			_ => unreachable!("subcommands were covered"),
		},
		_ => unreachable!("or todo"),
	}

	Ok(())
}

async fn start_hivemind() -> anyhow::Result<()> {
	let nats_url = env::var("NATS_URL").unwrap_or("nats://localhost:4222".to_string());
	let client = connect(nats_url).await?;
	let jetstream = jetstream::new(client.clone());

	let tasks_stream = jetstream
		.get_or_create_stream(jetstream::stream::Config {
			name: "tasks-stream".to_string(),
			discard: DiscardPolicy::New,
			subjects: vec!["tasks".to_string()],
			..Default::default()
		})
		.await?;
	let tasks_stream_consumer = tasks_stream
		.get_or_create_consumer(
			"pull",
			consumer::pull::Config {
				..Default::default()
			},
		)
		.await?;

	let task_schema = Schema::parse_str(TASK_RAW_SCHEMA)?;

	let mut tasks = tasks_stream_consumer.stream().messages().await?;
	while let Some(task) = tasks.next().await {
		let record = task?;
		let data = record.payload.clone();
		let reader = Reader::with_schema(&task_schema, &data[..])?;
		for value in reader {
			let record = value?;
			let task: Task = from_value(&record)?;
			println!("Task: {:?}", task);
		}
	}

	Ok(())
}
