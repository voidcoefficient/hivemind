use std::{env, process::exit, str::FromStr};

use futures::executor::block_on;
use hvmd::assets::{CreateAsset, db};
use seahorse::{App, Command, Context, Flag, FlagType};
use uuid::Uuid;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let args: Vec<String> = env::args().collect();
	let app = App::new("hm")
		.description(env!("CARGO_PKG_DESCRIPTION"))
		.author(env!("CARGO_PKG_AUTHORS"))
		.version(env!("CARGO_PKG_VERSION"))
		.usage("hm [args]")
		.command(asset_command());

	app.run(args);
	Ok(())
}

fn asset_command() -> Command {
	Command::new("asset")
		.description("an object with an amount")
		.alias("a")
		.usage("hm asset(a) [subcommand]")
		.command(asset_add_command())
		.command(asset_get_command())
		.command(asset_list_command())
}

fn asset_add_command() -> Command {
	Command::new("add")
		.description("create an asset")
		.alias("a")
		.usage("hm asset(a) add(a) [your asset title]")
		.flag(Flag::new("description", FlagType::String).alias("d"))
		.flag(Flag::new("amount", FlagType::Int).alias("a"))
		.action(asset_add_action)
}

fn asset_add_action(c: &Context) {
	if c.args.len() < 1 {
		eprintln!("wrong amount of arguments passed\n");
		c.help();
		exit(1);
	}

	let title = c.args.join(" ");
	let description = c.string_flag("description").ok();
	let amount = c.int_flag("amount").ok().map(|amount| amount as i32);
	match block_on(db::insert(CreateAsset {
		title,
		description,
		amount,
	})) {
		Ok(id) => println!("successfully created asset of id \"{}\"", id),
		Err(e) => eprintln!("{}", e),
	}
}

fn asset_get_command() -> Command {
	Command::new("get")
		.description("get one asset")
		.alias("g")
		.usage("hm asset(a) get(g) [uuid]")
		.action(asset_get_action)
}

fn asset_get_action(c: &Context) {
	if c.args.len() != 1 {
		eprintln!("wrong amount of arguments passed. try running `hm asset get --help`");
		exit(1);
	}

	let id = &(c.args[0]);
	match Uuid::from_str(id) {
		Ok(uuid) => match block_on(db::get(uuid)) {
			Ok(asset) => println!("{}", asset),
			Err(e) => eprintln!("{}", e),
		},
		Err(e) => eprintln!("{}", e),
	}
}

fn asset_list_command() -> Command {
	Command::new("list")
		.description("list assets")
		.alias("l")
		.alias("ls")
		.usage("hm asset(a) list(l)")
		.action(asset_list_action)
}

fn asset_list_action(c: &Context) {
	if c.args.len() != 0 {
		eprintln!("wrong amount of arguments passed. try running `hm asset list --help`");
		exit(1);
	}

	match block_on(db::list()) {
		Ok(assets) => assets.iter().for_each(|asset| println!("{}", asset)),
		Err(e) => eprintln!("{}", e),
	};
}
