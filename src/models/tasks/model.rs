use super::model::Entity as Task;
use crate::models::tags::model::Entity as Tag;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Default, Clone, DeriveEntityModel)]
#[sea_orm(table_name = "tasks")]
pub struct Model {
	#[sea_orm(primary_key)]
	pub id: Uuid,
	pub title: String,
	pub description: Option<String>,
	pub completed: bool,
	pub created_at: DateTime,
	pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(has_many = "crate::models::tags::model::Entity")]
	Tag,
}

impl Related<Task> for Tag {
	fn to() -> RelationDef {
		Relation::Tag.def()
	}
}

impl ActiveModelBehavior for ActiveModel {}
