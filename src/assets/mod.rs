use std::fmt::Display;

use serde::{Deserialize, Serialize};
use sqlx::types::time::PrimitiveDateTime;
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetAsset {
	pub id: Uuid,
	pub title: String,
	pub description: Option<String>,
	pub amount: i32,
	pub created_at: PrimitiveDateTime,
	pub updated_at: PrimitiveDateTime,
}

impl Display for GetAsset {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let description = self.description.clone().unwrap_or("-".to_string());
		write!(
			f,
			"{}\t{}\t{}\t{}\t{}\t{}",
			self.id, self.title, description, self.amount, self.created_at, self.updated_at,
		)
	}
}
