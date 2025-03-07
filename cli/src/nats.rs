use std::env;

use async_nats::{Client, connect};

pub async fn get_client() -> Result<Client, async_nats::error::Error<async_nats::ConnectErrorKind>>
{
	let nats_url = env::var("NATS_URL").unwrap_or("nats://localhost:4222".to_string());
	connect(nats_url).await
}
