use std::env;

use hvmd::commands::{asset::asset_command, task::task_command};
use seahorse::App;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let args: Vec<String> = env::args().collect();
	let app = App::new("hm")
		.description(env!("CARGO_PKG_DESCRIPTION"))
		.author(env!("CARGO_PKG_AUTHORS"))
		.version(env!("CARGO_PKG_VERSION"))
		.usage("hm [args]")
		.command(asset_command())
		.command(task_command());

	app.run(args);
	Ok(())
}
