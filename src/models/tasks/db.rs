use std::process::exit;

use itertools::Itertools;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QueryOrder};
use uuid::Uuid;

use crate::db::db;

use super::model::{ActiveModel, Column, Entity as Task, Model};
use super::{CreateTask, EditLastTask, EditTask, GetTask};
use crate::models::tags::model::Entity as Tag;

pub async fn insert(create_task: CreateTask) -> String {
	let db = &db().await;
	let task: ActiveModel = ActiveModel {
		id: Set(Uuid::new_v4()),
		title: Set(create_task.title),
		description: Set(create_task.description),
		completed: Set(create_task.completed.unwrap_or_default()),
		..Default::default()
	};

	match task.insert(db).await {
		Ok(row) => row.id.to_string(),
		Err(e) => {
			eprintln!("{}", e);
			exit(1);
		}
	}
}

pub async fn update(edit_task: EditTask) -> String {
	_update(edit_task, false).await
}

pub async fn update_last(edit_task: EditLastTask) -> String {
	_update(
		EditTask {
			title: edit_task.title,
			description: edit_task.description,
			completed: edit_task.completed,
			..Default::default()
		},
		true,
	)
	.await
}

async fn _update(edit_task: EditTask, is_last: bool) -> String {
	let db = &db().await;
	let command = if is_last {
		Task::find().order_by_desc(Column::UpdatedAt)
	} else {
		Task::find_by_id(edit_task.id)
	};

	match command.one(db).await {
		Ok(row) => match row {
			Some(row) => {
				let mut task: super::model::ActiveModel = row.into();

				if let Some(title) = edit_task.title {
					task.title = Set(title);
				}
				if let Some(description) = edit_task.description {
					task.description = Set(Some(description));
				}
				if let Some(completed) = edit_task.completed {
					task.completed = Set(completed);
				}

				match task.update(db).await {
					Ok(new_row) => new_row.id.to_string(),
					Err(e) => {
						eprintln!("{}", e);
						exit(1);
					}
				}
			}
			None => {
				eprintln!("could not find any tasks");
				exit(1);
			}
		},
		Err(e) => {
			eprintln!("{}", e);
			exit(1);
		}
	}
}

pub async fn get(id: Uuid) -> Option<Model> {
	match Task::find_by_id(id).one(&db().await).await {
		Ok(task) => task,
		Err(e) => {
			eprintln!("{}", e);
			exit(1);
		}
	}
}

/// Returns the last created or updated task.
/// If there's no tasks, returns `None`.
pub async fn get_last() -> Option<Model> {
	match Task::find()
		.order_by_desc(Column::UpdatedAt)
		.one(&db().await)
		.await
	{
		Ok(task) => task,
		Err(e) => {
			eprintln!("{}", e);
			exit(1);
		}
	}
}

pub async fn list() -> Vec<GetTask> {
	match Task::find()
		.find_also_related(Tag)
		.order_by_desc(Column::UpdatedAt)
		.all(&db().await)
		.await
	{
		Ok(task) => task.iter().map(GetTask::from).collect_vec(),
		Err(e) => {
			eprintln!("{}", e);
			exit(1);
		}
	}
}

pub async fn list_by_completed(completed: bool) -> Vec<Model> {
	match Task::find()
		.order_by_desc(Column::UpdatedAt)
		.filter(Column::Completed.eq(completed))
		.all(&db().await)
		.await
	{
		Ok(task) => task,
		Err(e) => {
			eprintln!("{}", e);
			exit(1);
		}
	}
}
