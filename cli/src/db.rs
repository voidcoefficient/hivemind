use std::{env, process::exit};

use sea_orm::{Database, DatabaseConnection};

pub async fn db() -> DatabaseConnection {
	match env::var("DATABASE_URL") {
		Ok(url) => match Database::connect(&url).await {
			Ok(db) => db,
			Err(err) => {
				eprintln!("{}", err);
				exit(1);
			}
		},
		_ => {
			eprintln!("could not retrieve $DATABASE_URL");
			exit(1);
		}
	}
}
