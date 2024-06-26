use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{Connection, PgConnection};

pub fn connect(database_url: &str) -> PgConnection {
	return PgConnection::establish(database_url)
		.unwrap_or_else(|_| panic!("Failed to connect to a database with url: {}", database_url));
}

pub fn pool(database_url: &str) -> Pool<ConnectionManager<PgConnection>> {
	let manager: ConnectionManager<PgConnection> =
		ConnectionManager::<PgConnection>::new(database_url);

	return Pool::builder()
		.max_size(7)
		.test_on_check_out(true)
		.build(manager)
		.expect("Could not build connection pool");
}
