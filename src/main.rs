pub mod tasks;

use std::env;

use apache_avro::{Reader, Schema, Writer, from_value};
use async_nats::{
	connect,
	jetstream::{self, consumer, stream::DiscardPolicy},
};
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
	let mut writer = Writer::with_codec(&task_schema, Vec::new(), apache_avro::Codec::Deflate);

	let added_task = Task {
		title: "My task".to_owned(),
		..Default::default()
	};
	writer.append_ser(added_task)?;
	let encoded_task = writer.into_inner()?;
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
