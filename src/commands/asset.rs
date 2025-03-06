use std::{process::exit, str::FromStr};

use futures::executor::block_on;
use itertools::Itertools;
use seahorse::{Command, Context, Flag, FlagType};
use uuid::Uuid;

use crate::assets::{CreateAsset, EditAsset, db};

pub fn asset_command() -> Command {
	Command::new("asset")
		.description("object with an amount")
		.alias("a")
		.usage("hm asset(a) [subcommand]")
		.command(asset_add_command())
		.command(asset_edit_command())
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
	if c.args.is_empty() {
		eprintln!("wrong amount of arguments passed\n");
		c.help();
		exit(1);
	}

	let title = c.args.join(" ");
	let description = c.string_flag("description").ok();
	let amount = c.int_flag("amount").ok().map(|amount| amount as i32);
	let id = block_on(db::insert(CreateAsset {
		title,
		description,
		amount,
	}));
	println!("successfully created asset of id \"{}\"", id);
}

fn asset_edit_command() -> Command {
	Command::new("edit")
		.description("edit an asset")
		.alias("e")
		.usage("hm asset(a) edit(e) [uuid]")
		.flag(Flag::new("title", FlagType::String).alias("t"))
		.flag(Flag::new("description", FlagType::String).alias("d"))
		.flag(Flag::new("amount", FlagType::Int).alias("a"))
		.action(asset_edit_action)
}

fn asset_edit_action(c: &Context) {
	if c.args.len() != 1 {
		eprintln!("wrong amount of arguments passed\n");
		c.help();
		exit(1);
	}

	match Uuid::from_str(&c.args[0]) {
		Ok(id) => {
			let title = c.string_flag("title").ok();
			let description = c.string_flag("description").ok();
			let amount = c.int_flag("amount").ok().map(|amount| amount as i32);
			let id = block_on(db::update(EditAsset {
				id,
				title,
				description,
				amount,
			}));
			println!("successfully updated asset \"{}\"", id);
		}
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
			Some(asset) => println!("{}", asset),
			None => eprintln!("could not find asset \"{}\"", id),
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
	if !c.args.is_empty() {
		eprintln!("wrong amount of arguments passed. try running `hm asset list --help`");
		exit(1);
	}

	println!("{}", block_on(db::list()).iter().join("\n\n"));
}
