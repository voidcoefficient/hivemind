use std::fmt::Display;

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Default, Clone, DeriveEntityModel)]
#[sea_orm(table_name = "assets")]
pub struct Model {
	#[sea_orm(primary_key)]
	pub id: Uuid,
	pub title: String,
	pub description: Option<String>,
	//  TODO: pub tags: Vec<Tag>,
	pub amount: i32,
	pub created_at: DateTime,
	pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Display for Model {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let description = self.description.clone().unwrap_or("".to_string());
		write!(
			f,
			"asset\t\t{}\ntitle\t\t{}\ndescription\t{}\namount\t\t{}\ncreated at\t{}\nupdated at\t{}",
			self.id, self.title, description, self.amount, self.created_at, self.updated_at,
		)
	}
}
