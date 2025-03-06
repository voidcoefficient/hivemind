use std::env;

use hvmd::commands::{asset::asset_command, task::task_command};
use seahorse::{App, Flag, FlagType};
use tracing::Level;
use tracing_subscriber::fmt;

fn setup_tracing(debug: bool) {
	let level = if debug { Level::DEBUG } else { Level::ERROR };
	let logger = fmt().with_ansi(true).with_max_level(level).pretty();

	if !debug {
		logger.init();
		return;
	}

	logger
		.with_target(true)
		.with_file(true)
		.with_timer(fmt::time::Uptime::default())
		.init();
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let args: Vec<String> = env::args().collect();
	let app = App::new("hm")
		.description(env!("CARGO_PKG_DESCRIPTION"))
		.author(env!("CARGO_PKG_AUTHORS"))
		.version(env!("CARGO_PKG_VERSION"))
		.usage("hm [args]")
		.command(asset_command())
		.command(task_command())
		.flag(Flag::new("debug", FlagType::Bool).description("enables debugging information"));

	let debug = args.iter().any(|arg| arg == "--debug");
	setup_tracing(debug);
	app.run(args);
	Ok(())
}
