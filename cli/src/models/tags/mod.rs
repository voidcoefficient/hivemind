use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod db;
pub mod model;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CreateTag {
	pub title: String,
	pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct EditTag {
	pub id: Uuid,
	pub title: Option<String>,
	pub description: Option<String>,
}
