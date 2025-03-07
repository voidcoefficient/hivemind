use std::fmt::Display;

use sea_orm::entity::prelude::DateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::tags::model::Entity as Tag;
use crate::models::tasks::model::Model as Task;

pub mod db;
pub mod model;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CreateTask {
	pub title: String,
	pub description: Option<String>,
	pub completed: Option<bool>,
	pub tags: Vec<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct EditTask {
	pub id: Uuid,
	pub title: Option<String>,
	pub description: Option<String>,
	pub completed: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct EditLastTask {
	pub title: Option<String>,
	pub description: Option<String>,
	pub completed: Option<bool>,
}

#[derive(Debug, Default, Clone)]
pub struct GetTask {
	pub id: Uuid,
	pub title: String,
	pub description: Option<String>,
	pub completed: bool,
	pub tags: Vec<Tag>,
	pub created_at: DateTime,
	pub updated_at: DateTime,
}

impl From<&Task> for GetTask {
	fn from(model: &Task) -> GetTask {
		GetTask {
			id: model.id,
			title: model.title.clone(),
			description: model.description.clone(),
			completed: model.completed,
			tags: vec![],
			created_at: model.created_at,
			updated_at: model.updated_at,
		}
	}
}

impl Display for GetTask {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let description = self.description.clone().unwrap_or("".to_string());
		write!(
			f,
			"asset\t\t{}\ntitle\t\t{}\ndescription\t{}\ncompleted\t{}\ncreated at\t{}\nupdated at\t{}",
			self.id, self.title, description, self.completed, self.created_at, self.updated_at,
		)
	}
}
