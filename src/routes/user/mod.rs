pub mod types;

use actix_web::{web, HttpResponse, Responder};

pub fn config(cfg: &mut web::ServiceConfig) {
	cfg.service(web::scope("/user").route("/create", web::get().to(create)));
}

async fn create() -> impl Responder {
	return HttpResponse::Ok().body("hahahah");
}
