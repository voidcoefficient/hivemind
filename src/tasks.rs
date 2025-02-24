use apache_avro::Schema;
use bytes::Bytes;
use serde::{Deserialize, Serialize};

use crate::{TASK_RAW_SCHEMA, avro::serialize_avro};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Task {
	pub id: uuid::Uuid,
	pub title: String,
	pub description: Option<String>,
	pub completed: bool,
	pub created_at: i64,
	pub updated_at: i64,
	pub completed_at: Option<i64>,
}

pub fn serialize_task(task: Task) -> Result<Bytes, apache_avro::Error> {
	let schema = Schema::parse_str(TASK_RAW_SCHEMA)?;
	serialize_avro(&schema, task).map(|task| task.into())
}
