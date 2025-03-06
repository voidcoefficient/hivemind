use std::process::exit;

use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, EntityTrait};
use uuid::Uuid;

use crate::db::db;

use super::model::{ActiveModel, Entity as Asset, Model};
use super::{CreateTask, EditTask};

pub async fn insert(create_asset: CreateTask) -> String {
	let db = &db().await;
	let asset: ActiveModel = ActiveModel {
		id: Set(Uuid::new_v4()),
		title: Set(create_asset.title),
		description: Set(create_asset.description),
		completed: Set(create_asset.completed.unwrap_or_default()),
		..Default::default()
	};

	match asset.insert(db).await {
		Ok(row) => row.id.to_string(),
		Err(e) => {
			eprintln!("{}", e);
			exit(1);
		}
	}
}

pub async fn update(edit_asset: EditTask) -> String {
	let db = &db().await;
	match Asset::find_by_id(edit_asset.id).one(db).await {
		Ok(row) => match row {
			Some(row) => {
				let mut asset: super::model::ActiveModel = row.into();

				if let Some(title) = edit_asset.title {
					asset.title = Set(title);
				}
				if let Some(description) = edit_asset.description {
					asset.description = Set(Some(description));
				}
				if let Some(completed) = edit_asset.completed {
					asset.completed = Set(completed);
				}

				match asset.update(db).await {
					Ok(new_row) => new_row.id.to_string(),
					Err(e) => {
						eprintln!("{}", e);
						exit(1);
					}
				}
			}
			None => {
				eprintln!("could not find asset of #{}", edit_asset.id);
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
	match Asset::find_by_id(id).one(&db().await).await {
		Ok(asset) => asset,
		Err(e) => {
			eprintln!("{}", e);
			exit(1);
		}
	}
}

pub async fn list() -> Vec<Model> {
	match Asset::find().all(&db().await).await {
		Ok(assets) => assets,
		Err(e) => {
			eprintln!("{}", e);
			exit(1);
		}
	}
}
