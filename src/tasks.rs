use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Task {
	pub title: String,
	pub description: Option<String>,
	pub completed: bool,
}
