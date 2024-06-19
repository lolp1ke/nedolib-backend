use sqlx::{postgres::PgPoolOptions, PgPool};

pub async fn connect(url: &str) -> PgPool {
	return PgPoolOptions::new()
		.max_connections(7)
		.connect(url)
		.await
		.unwrap();
}
