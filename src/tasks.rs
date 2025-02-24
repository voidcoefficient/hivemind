use apache_avro::{Reader, Schema, from_value};
use bytes::Bytes;
use serde::{Deserialize, Serialize};

use crate::avro::serialize_avro;

pub const TASK_RAW_SCHEMA: &str = r#"
  {
    "type": "record",
    "name": "task",
    "fields": [
      { "name": "id", "type": "string", "logicalType": "uuid" },
      { "name": "title", "type": "string" },
      { "name": "description", "type": ["null", "string"], "default": null },
      { "name": "completed", "type": "boolean", "default": false },
      { "name": "created_at", "type": "long", "logicalType": "local-timestamp-millis" },
      { "name": "updated_at", "type": "long", "logicalType": "local-timestamp-millis" },
      { "name": "completed_at", "type": ["null", "long"], "logicalType": "local-timestamp-millis", "default": null }
    ]
  }
"#;

pub const CREATE_TASK_RAW_SCHEMA: &str = r#"
  {
    "type": "record",
    "name": "task.create",
    "fields": [
      { "name": "title", "type": "string" },
      { "name": "description", "type": ["null", "string"], "default": null },
      { "name": "completed", "type": "boolean", "default": false }
    ]
  }
"#;

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

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CreateTask {
	pub title: String,
	pub description: Option<String>,
	pub completed: bool,
}

pub fn serialize<T: Serialize>(value: T, schema: &Schema) -> Result<Bytes, apache_avro::Error> {
	serialize_avro(schema, value).map(|task| task.into())
}

pub fn deserialize<T>(data: &Bytes, schema: &Schema) -> Option<T>
where
	T: for<'a> Deserialize<'a>,
{
	let reader = Reader::with_schema(schema, &data[..]).ok()?;
	let mut reader = reader.peekable();
	let value = reader.peek()?.as_ref().ok()?;
	from_value(value).ok()
}

pub trait Model<T>
where
	T: for<'a> Deserialize<'a>,
	T: Serialize,
{
	fn schema() -> Schema;
	fn serialize(value: T) -> Result<Bytes, apache_avro::Error>;
	fn deserialize(data: Bytes) -> Option<T>;
}

pub struct TaskSerde;
impl Model<Task> for TaskSerde {
	fn schema() -> Schema {
		Schema::parse_str(TASK_RAW_SCHEMA).unwrap()
	}

	fn serialize(value: Task) -> Result<Bytes, apache_avro::Error> {
		let schema = Schema::parse_str(TASK_RAW_SCHEMA)?;
		serialize(value, &schema)
	}

	fn deserialize(data: Bytes) -> Option<Task> {
		let schema = TaskSerde::schema();
		deserialize(&data, &schema)
	}
}

pub struct CreateTaskSerde;
impl Model<CreateTask> for CreateTaskSerde {
	fn schema() -> Schema {
		Schema::parse_str(CREATE_TASK_RAW_SCHEMA).unwrap()
	}

	fn serialize(value: CreateTask) -> Result<Bytes, apache_avro::Error> {
		let schema = Schema::parse_str(CREATE_TASK_RAW_SCHEMA)?;
		serialize(value, &schema)
	}

	fn deserialize(data: Bytes) -> Option<CreateTask> {
		let schema = CreateTaskSerde::schema();
		deserialize(&data, &schema)
	}
}
