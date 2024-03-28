pub mod routes;

use actix_web::{
	web::{self, ServiceConfig},
	App, HttpServer,
};
use dotenvy::dotenv;
use env_logger::Env;
use log::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	dotenv().ok();
	env_logger::Builder::from_env(Env::default())
		.format_indent(Some(2))
		.init();


	let env: String = dotenvy::var("ENVIRONMENT").expect("Set up ENVIRONMENT at .env file");
	if env == "PRODUCTION" {
		dotenvy::from_filename_override(".env.production").ok();
	} else if env == "DEVELOPMENT" {
		dotenvy::from_filename_override(".env.development").ok();
	}

	let host: String = dotenvy::var("BACKEND_HOST").expect("Set up host address at .env file");
	let port: u16 = dotenvy::var("BACKEND_PORT")
		.expect("Set up port address at .env file")
		.parse()
		.expect("Port must be a positive number");


	info!("Server listening at address - {host}:{port}");
	return HttpServer::new(move || {
		return App::new().configure(move |cfg: &mut ServiceConfig| {
			cfg.service(web::scope("/api").configure(routes::v1::config));
		});
	})
	.bind((host, port))?
	.run()
	.await;
}
