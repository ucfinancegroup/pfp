pub use crate::models::user_model;

use actix_web::{Responder, HttpResponse, post};

#[post("/signup")]
pub async fn signup() -> impl Responder {
  HttpResponse::Ok().body("hi")
}

use actix_web::web::ServiceConfig;
pub fn init_routes(config: &mut ServiceConfig) {
  config.service(signup);
}
