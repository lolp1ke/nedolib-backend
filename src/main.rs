mod database;
mod routes;

use actix_web::{web, App, HttpServer};
use diesel::{
	r2d2::{ConnectionManager, Pool},
	PgConnection,
};
use log::info;
use nedolib_backend::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	dotenvy::dotenv().ok();
	env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();


	let host: String =
		dotenvy::var("BACKEND_HOST").expect("Set up {BACKEND_HOST} variable in .env file");
	let port: u16 = dotenvy::var("BACKEND_PORT")
		.expect("Set up {BACKEND_PORT} variable in .env file")
		.parse::<u16>()
		.expect("Port must be a positive integer");


	let database_url: String =
		dotenvy::var("DATABASE_URL").expect("Set up {DATABASE_URL} variable in .env file");
	let db_pool: Pool<ConnectionManager<PgConnection>> = database::pool(&database_url);


	info!("Listening on address: {}:{}", host, port);
	return HttpServer::new(move || {
		return App::new()
			.app_data(web::Data::new(AppState::new(db_pool.clone())))
			.configure(nedolib_backend::app_config);
	})
	.bind((host, port))?
	.run()
	.await;
}
