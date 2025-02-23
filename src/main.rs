pub mod avro;
pub mod tasks;

use std::env;

use apache_avro::{Reader, Schema, from_value};
use async_nats::{
	connect,
	jetstream::{self, consumer, stream::DiscardPolicy},
};
use avro::serialize_avro;
use futures::StreamExt;
use tasks::Task;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
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

	let task_raw_schema = r#"
    {
        "type": "record",
        "name": "task",
        "fields": [
            { "name": "title", "type": "string" },
            { "name": "description", "type": ["null", "string"], "default": null },
            { "name": "completed", "type": "boolean", "default": false }
        ]
    }
    "#;
	let task_schema = Schema::parse_str(task_raw_schema)?;

	let added_task = Task {
		title: "My task".to_owned(),
		..Default::default()
	};
	let encoded_task = serialize_avro(&task_schema, added_task)?;
	client.publish("tasks", encoded_task.into()).await?;

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
