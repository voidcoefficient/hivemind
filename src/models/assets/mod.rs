use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod db;
pub mod model;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CreateAsset {
	pub title: String,
	pub description: Option<String>,
	pub amount: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct EditAsset {
	pub id: Uuid,
	pub title: Option<String>,
	pub description: Option<String>,
	pub amount: Option<i32>,
}
