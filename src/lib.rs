use actix_web::web;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
	pub db_pool: PgPool,
}
impl AppState {
	pub fn new(db_pool: PgPool) -> Self {
		return Self { db_pool };
	}
}

pub fn env_error(var_name: &str) -> String {
	return format!("Set up {var_name} variable in .env file");
}

pub fn app_config(cfg: &mut web::ServiceConfig) {
	// cfg.configure(f)
	todo!();
}
