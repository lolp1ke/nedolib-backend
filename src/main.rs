use actix_web::{App, HttpServer};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
	dotenvy::dotenv().ok();

	let host: String = dotenvy::var("BACKEND_HOST").expect("Set up BACKEND_HOST var in .env");
	let port: u16 = dotenvy::var("BACKEND_PORT")
		.expect("Set up BACKEND_PORT var in .env")
		.parse()
		.expect("Port must be a positive integer");

	return HttpServer::new(|| {
		return App::new();
	})
	.bind((host, port))?
	.run()
	.await;
}
