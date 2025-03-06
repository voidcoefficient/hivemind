use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod db;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Asset {
	pub id: Uuid,
	pub title: String,
	pub description: Option<String>,
	// TODO: pub location: Location,
	// TODO: tags
	// pub tags: Vec<String>,
	// pub tags: Vec<Tag>,
	pub amount: i32,
	pub created_at: i64,
	pub updated_at: i64,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CreateAsset {
	pub title: String,
	pub description: Option<String>,
}
