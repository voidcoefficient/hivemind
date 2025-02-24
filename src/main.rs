pub mod avro;
pub mod nats;
pub mod process;
pub mod tasks;
use apache_avro::{Reader, Schema, from_value};
use async_nats::service::{self, ServiceExt};
use chrono::Local;
use clap::{ArgAction, arg, command, crate_authors};
use futures::{StreamExt, future::join};
use nats::get_client;
use process::TaskProcessor;
use tasks::{CreateTask, CreateTaskSerde, Model, Task};

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
				),
		)
		.get_matches();

	let client = get_client().await?;
	match matches.subcommand() {
		Some(subcommand) => match subcommand {
			("start", _sub_matches) => {
				let processor = TaskProcessor::new(client);
				join(processor.process_tasks(), task_service()).await;
			}
			("tasks", sub_matches) => match sub_matches.subcommand() {
				Some(("create", sub_matches)) => {
					let title = sub_matches.get_one::<String>("title").unwrap().to_owned();
					let description = sub_matches
						.get_one::<String>("description")
						.map(|d| d.to_owned());
					let completed = sub_matches.get_flag("completed");

					let task = CreateTask {
						title,
						description,
						completed,
					};
					let encoded_task = CreateTaskSerde::serialize(task.clone())?;

					client.request("tasks.create", encoded_task).await?;

					println!("created task \"{}\"", task.title);
				}
				_ => unreachable!("subcommands were covered"),
			},
			_ => unreachable!("subcommands were covered"),
		},
		_ => unreachable!("or todo"),
	}

	Ok(())
}

async fn task_service() -> anyhow::Result<()> {
	let client = get_client().await?;
	let service = client
		.add_service(service::Config {
			name: "tasks".to_owned(),
			version: "0.1.0".to_owned(),
			description: None,
			stats_handler: None,
			metadata: None,
			queue_group: None,
		})
		.await
		.expect("should happen");
	let tasks_group = service.group("tasks");
	let mut endpoint = tasks_group.endpoint("create").await.expect("should happen");

	while let Some(request) = endpoint.next().await {
		let data = request.message.payload.clone();
		match CreateTaskSerde::deserialize(data) {
			Some(task) => {
				let serialized_task = CreateTaskSerde::serialize(task)?;
				client.publish("tasks", serialized_task).await?;
				request.respond(Ok("200".into())).await.unwrap();
			}
			None => request.respond(Ok("error".into())).await?,
		}
	}

	Ok(())
}
