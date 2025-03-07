use std::{process::exit, str::FromStr};

use futures::executor::block_on;
use itertools::Itertools;
use seahorse::{Command, Context, Flag, FlagType};
use uuid::Uuid;

use crate::models::tags::{CreateTag, EditTag, db};

pub fn tag_command() -> Command {
	Command::new("tag")
		.description("extra metadata for assets, tasks, etc.")
		.usage("hm tag [subcommand]")
		.command(tag_add_command())
		.command(tag_edit_command())
		.command(tag_get_command())
		.command(tag_list_command())
}

fn tag_add_command() -> Command {
	Command::new("add")
		.description("create a tag")
		.alias("a")
		.usage("hm tag add(a) [your tag title]")
		.flag(Flag::new("description", FlagType::String).alias("d"))
		.action(tag_add_action)
}

fn tag_add_action(c: &Context) {
	if c.args.is_empty() {
		eprintln!("wrong amount of arguments passed\n");
		c.help();
		exit(1);
	}

	let title = c.args.join(" ");
	let description = c.string_flag("description").ok();
	let id = block_on(db::insert(CreateTag { title, description }));
	println!("successfully created tag of id \"{}\"", id);
}

fn tag_edit_command() -> Command {
	Command::new("edit")
		.description("edit a tag")
		.alias("e")
		.usage("hm tag edit(e) [uuid]")
		.flag(Flag::new("title", FlagType::String).alias("t"))
		.flag(Flag::new("description", FlagType::String).alias("d"))
		.action(tag_edit_action)
}

fn tag_edit_action(c: &Context) {
	if c.args.is_empty() {
		eprintln!("wrong amount of arguments passed\n");
		c.help();
		exit(1);
	}

	let title = c.string_flag("title").ok();
	let description = c.string_flag("description").ok();

	match Uuid::from_str(&c.args[0]) {
		Ok(id) => {
			let id = block_on(db::update(EditTag {
				id,
				title,
				description,
			}));
			println!("successfully updated tag \"{}\"", id);
		}
		Err(e) => eprintln!("{}", e),
	}
}

fn tag_get_command() -> Command {
	Command::new("get")
		.description("get one tag")
		.alias("g")
		.usage("hm tag get(g) [uuid]")
		.action(tag_get_action)
}

fn tag_get_action(c: &Context) {
	if c.args.len() != 1 {
		eprintln!("wrong amount of arguments passed\n");
		c.help();
		exit(1);
	}

	let id = &(c.args[0]);
	match Uuid::from_str(id) {
		Ok(uuid) => match block_on(db::get(uuid)) {
			Some(tag) => println!("{}", tag),
			None => eprintln!("could not find tag \"{}\"", id),
		},
		Err(e) => eprintln!("{}", e),
	}
}

fn tag_list_command() -> Command {
	Command::new("list")
		.description("list tags")
		.alias("l")
		.alias("ls")
		.usage("hm tag list(l)")
		.action(tag_list_action)
}

fn tag_list_action(_c: &Context) {
	println!("{}", block_on(db::list()).iter().join("\n\n"));
}
