use std::env;

use async_nats::{
    connect,
    jetstream::{self, consumer, stream::DiscardPolicy},
};
use futures::StreamExt;

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

    let mut tasks = tasks_stream_consumer.stream().messages().await?;
    while let Some(task) = tasks.next().await {
        let task = task?;
        println!("Task: {:?}", task.payload);
    }

    Ok(())
}
