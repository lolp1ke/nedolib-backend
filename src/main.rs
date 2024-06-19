mod database;

use actix_web::{web, App, HttpServer};
use log::info;
use nedolib_backend::{env_error, AppState};
use sqlx::PgPool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	dotenvy::dotenv().ok();
	env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();


	let host: String = dotenvy::var("BACKEND_HOST").expect(&env_error("BACKEND_HOST"));
	let port: u16 = dotenvy::var("BACKEND_PORT")
		.expect(&env_error("BACKEND_PORT"))
		.parse::<u16>()
		.expect("Port must be a positive integer");


	let database_url: String = dotenvy::var("DATABASE_URL").expect(&env_error("DATABASE_URL"));
	let db_pool: PgPool = database::connect(&database_url).await;


	let app_state: AppState = AppState::new(db_pool);


	info!("Listening on address: {host}:{port}");
	return HttpServer::new(move || {
		return App::new()
			.app_data(web::Data::new(app_state.clone()))
			.configure(nedolib_backend::app_config);
	})
	.bind((host, port))?
	.run()
	.await;
}
