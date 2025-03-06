use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod db;
pub mod model;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CreateTask {
	pub title: String,
	pub description: Option<String>,
	pub completed: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct EditTask {
	pub id: Uuid,
	pub title: Option<String>,
	pub description: Option<String>,
	pub completed: Option<bool>,
}
