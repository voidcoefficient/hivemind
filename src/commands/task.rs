use std::{process::exit, str::FromStr};

use futures::executor::block_on;
use itertools::Itertools;
use seahorse::{Command, Context, Flag, FlagType};
use uuid::Uuid;

use crate::tasks::{CreateTask, EditLastTask, EditTask, db};

pub fn task_command() -> Command {
	Command::new("task")
		.description("work to be done")
		.alias("t")
		.usage("hm task(t) [subcommand]")
		.command(task_add_command())
		.command(task_edit_command())
		.command(task_done_command())
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
	if c.args.is_empty() {
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
		.flag(
			Flag::new("last", FlagType::Bool)
				.alias("l")
				.description("edit the last task"),
		)
		.flag(Flag::new("title", FlagType::String).alias("t"))
		.flag(Flag::new("description", FlagType::String).alias("d"))
		.flag(Flag::new("completed", FlagType::Bool).alias("c"))
		.action(task_edit_action)
}

fn task_edit_action(c: &Context) {
	let is_last = c.bool_flag("last");
	if c.args.is_empty() && !is_last {
		eprintln!("wrong amount of arguments passed\n");
		c.help();
		exit(1);
	}

	let title = c.string_flag("title").ok();
	let description = c.string_flag("description").ok();
	let completed = Some(c.bool_flag("completed"));

	if is_last {
		let id = block_on(db::update_last(EditLastTask {
			title,
			description,
			completed,
		}));
		println!("successfully updated task \"{}\"", id);
		exit(0);
	}

	match Uuid::from_str(&c.args[0]) {
		Ok(id) => {
			let id = block_on(db::update(EditTask {
				id,
				title,
				description,
				completed,
			}));
			println!("successfully updated task \"{}\"", id);
		}
		Err(e) => eprintln!("{}", e),
	}
}

fn task_done_command() -> Command {
	Command::new("done")
		.description("mark a task as completed")
		.alias("d")
		.usage("hm task(t) done(d) [uuid]")
		.action(task_done_action)
}

fn task_done_action(c: &Context) {
	if c.args.len() != 1 {
		eprintln!("wrong amount of arguments passed\n");
		c.help();
		exit(1);
	}

	match Uuid::from_str(&c.args[0]) {
		Ok(id) => {
			let id = block_on(db::update(EditTask {
				id,
				title: None,
				description: None,
				completed: Some(true),
			}));
			println!("successfully marked task \"{}\" as completed", id);
		}
		Err(e) => eprintln!("{}", e),
	}
}

fn task_get_command() -> Command {
	Command::new("get")
		.description("get one task")
		.alias("g")
		.usage("hm task(t) get(g) [uuid]")
		.flag(
			Flag::new("last", FlagType::Bool)
				.alias("l")
				.description("get the last created/updated task"),
		)
		.action(task_get_action)
}

fn task_get_action(c: &Context) {
	let is_last = c.bool_flag("last");
	if is_last {
		match block_on(db::get_last()) {
			Some(last_task) => {
				println!("{}", last_task);
				exit(0);
			}
			None => {
				eprintln!("could not find any task");
				exit(1);
			}
		}
	}

	if c.args.len() != 1 {
		eprintln!("wrong amount of arguments passed\n");
		c.help();
		exit(1);
	}

	let id = &(c.args[0]);
	match Uuid::from_str(id) {
		Ok(uuid) => match block_on(db::get(uuid)) {
			Some(task) => println!("{}", task),
			None => eprintln!("could not find task \"{}\"", id),
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
		.flag(Flag::new("completed", FlagType::Bool).description("filter by completed tasks"))
		.flag(Flag::new("pending", FlagType::Bool).description("filter by pending tasks"))
		.action(task_list_action)
}

fn task_list_action(c: &Context) {
	if c.bool_flag("completed") {
		println!(
			"{}",
			block_on(db::list_by_completed(true)).iter().join("\n\n")
		);
		exit(0);
	} else if c.bool_flag("pending") {
		println!(
			"{}",
			block_on(db::list_by_completed(false)).iter().join("\n\n")
		);
		exit(0);
	}

	println!("{}", block_on(db::list()).iter().join("\n\n"));
}
