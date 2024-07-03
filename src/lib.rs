mod routes;
mod schema;

use actix_web::web;
use diesel::{
	r2d2::{ConnectionManager, Pool},
	PgConnection,
};


#[derive(Clone)]
pub struct AppState {
	db_pool: Pool<ConnectionManager<PgConnection>>,
}
impl AppState {
	pub fn new(db_pool: Pool<ConnectionManager<PgConnection>>) -> Self {
		return Self { db_pool };
	}
}


pub fn app_config(cfg: &mut web::ServiceConfig, appState: &AppState) {
	cfg
		.app_data(web::Data::new(appState.db_pool.clone()))
		.configure(|cfg: &mut web::ServiceConfig| routes::config(cfg));
}
