use std::env;

use anyhow::Result;
use sqlx::{PgPool, query};
use uuid::Uuid;

use super::{Asset, CreateAsset};

pub async fn insert(asset: CreateAsset) -> Result<String> {
	let pool = &PgPool::connect(&env::var("DATABASE_URL")?).await?;
	let rec = query!(
		r#"insert into assets (id, title, description) values ($1, $2, $3) returning id"#,
		uuid::Uuid::new_v4(),
		asset.title,
		asset.description
	)
	.fetch_one(pool)
	.await?;

	Ok(rec.id.to_string())
}

pub async fn get(id: Uuid) -> Result<Asset> {
	let pool = &PgPool::connect(&env::var("DATABASE_URL")?).await?;
	let rec = query!(r#"select * from assets where id = $1"#, id,)
		.fetch_one(pool)
		.await?;

	Ok(Asset {
		id: rec.id,
		title: rec.title,
		description: rec.description,
		amount: rec.amount,
		created_at: 0,
		updated_at: 0,
	})
}

pub async fn list() -> Result<Vec<Asset>> {
	let pool = &PgPool::connect(&env::var("DATABASE_URL")?).await?;
	let rec = query!(r#"select * from assets"#).fetch_all(pool).await?;

	Ok(
		rec
			.iter()
			.map(|rec| Asset {
				id: rec.id,
				title: rec.title.clone(),
				description: rec.description.clone(),
				amount: rec.amount,
				created_at: 0,
				updated_at: 0,
			})
			.collect(),
	)
}
