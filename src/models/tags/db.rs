use std::process::exit;

use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, EntityTrait, QueryOrder};
use uuid::Uuid;

use crate::db::db;

use super::model::{ActiveModel, Column, Entity as Tag, Model};
use super::{CreateTag, EditTag};

pub async fn insert(create_tag: CreateTag) -> String {
	let db = &db().await;
	let tag: ActiveModel = ActiveModel {
		id: Set(Uuid::new_v4()),
		title: Set(create_tag.title),
		description: Set(create_tag.description),
		..Default::default()
	};

	match tag.insert(db).await {
		Ok(row) => row.id.to_string(),
		Err(e) => {
			eprintln!("{}", e);
			exit(1);
		}
	}
}

pub async fn update(edit_tag: EditTag) -> String {
	let db = &db().await;

	match Tag::find_by_id(edit_tag.id).one(db).await {
		Ok(row) => match row {
			Some(row) => {
				let mut tag: ActiveModel = row.into();

				if let Some(title) = edit_tag.title {
					tag.title = Set(title);
				}
				if let Some(description) = edit_tag.description {
					tag.description = Set(Some(description));
				}

				match tag.update(db).await {
					Ok(new_row) => new_row.id.to_string(),
					Err(e) => {
						eprintln!("{}", e);
						exit(1);
					}
				}
			}
			None => {
				eprintln!("could not find tag \"{}\"", edit_tag.id);
				exit(1);
			}
		},
		Err(e) => {
			eprintln!("{}", e);
			exit(1);
		}
	}
}

pub async fn get(id: Uuid) -> Option<Model> {
	match Tag::find_by_id(id).one(&db().await).await {
		Ok(tag) => tag,
		Err(e) => {
			eprintln!("{}", e);
			exit(1);
		}
	}
}

pub async fn list() -> Vec<Model> {
	match Tag::find()
		.order_by_desc(Column::UpdatedAt)
		.all(&db().await)
		.await
	{
		Ok(tags) => tags,
		Err(e) => {
			eprintln!("{}", e);
			exit(1);
		}
	}
}
