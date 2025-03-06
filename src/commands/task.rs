use std::{process::exit, str::FromStr};

use futures::executor::block_on;
use itertools::Itertools;
use seahorse::{Command, Context, Flag, FlagType};
use uuid::Uuid;

use crate::tasks::{CreateTask, EditTask, db};

pub fn task_command() -> Command {
	Command::new("task")
		.description("work to be done")
		.alias("t")
		.usage("hm task(t) [subcommand]")
		.command(task_add_command())
		.command(task_edit_command())
		.command(task_get_command())
		.command(task_list_command())
}

fn task_add_command() -> Command {
	Command::new("add")
		.description("create a task")
		.alias("a")
		.usage("hm task(t) add(a) [your task title]")
		.flag(Flag::new("description", FlagType::String).alias("d"))
		.flag(
			Flag::new("completed", FlagType::Bool)
				.alias("c")
				.description("if the task is completed already"),
		)
		.action(task_add_action)
}

fn task_add_action(c: &Context) {
	if c.args.len() < 1 {
		eprintln!("wrong amount of arguments passed\n");
		c.help();
		exit(1);
	}

	let title = c.args.join(" ");
	let description = c.string_flag("description").ok();
	let completed = Some(c.bool_flag("completed"));
	let id = block_on(db::insert(CreateTask {
		title,
		description,
		completed,
	}));
	println!("successfully created task of id \"{}\"", id);
}

fn task_edit_command() -> Command {
	Command::new("edit")
		.description("edit a task")
		.alias("e")
		.usage("hm task(t) edit(e) [uuid]")
		.flag(Flag::new("title", FlagType::String).alias("t"))
		.flag(Flag::new("description", FlagType::String).alias("d"))
		.flag(Flag::new("completed", FlagType::Bool).alias("c"))
		.action(task_edit_action)
}

fn task_edit_action(c: &Context) {
	if c.args.len() != 1 {
		eprintln!("wrong amount of arguments passed\n");
		c.help();
		exit(1);
	}

	match Uuid::from_str(&c.args[0]) {
		Ok(id) => {
			let title = c.string_flag("title").ok();
			let description = c.string_flag("description").ok();
			let completed = Some(c.bool_flag("completed"));
			let id = block_on(db::update(EditTask {
				id,
				title,
				description,
				completed,
			}));
			println!("successfully updated task #{}", id);
		}
		Err(e) => eprintln!("{}", e),
	}
}

fn task_get_command() -> Command {
	Command::new("get")
		.description("get one task")
		.alias("g")
		.usage("hm task(t) get(g) [uuid]")
		.action(task_get_action)
}

fn task_get_action(c: &Context) {
	if c.args.len() != 1 {
		eprintln!("wrong amount of arguments passed. try running `hm task get --help`");
		exit(1);
	}

	let id = &(c.args[0]);
	match Uuid::from_str(id) {
		Ok(uuid) => match block_on(db::get(uuid)) {
			Some(task) => println!("{}", task),
			None => eprintln!("could not find task #{}", id),
		},
		Err(e) => eprintln!("{}", e),
	}
}

fn task_list_command() -> Command {
	Command::new("list")
		.description("list tasks")
		.alias("l")
		.alias("ls")
		.usage("hm task(t) list(l)")
		.action(task_list_action)
}

fn task_list_action(c: &Context) {
	if c.args.len() != 0 {
		eprintln!("wrong amount of arguments passed. try running `hm task list --help`");
		exit(1);
	}

	println!("{}", block_on(db::list()).iter().join("\n\n"));
}
