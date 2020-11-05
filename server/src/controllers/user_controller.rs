pub use crate::models::user_model;

use serde::{Deserialize};
use actix_web::{Responder, HttpResponse, HttpRequest, web::Json, post, web::Data};
use actix_session::Session;

use rand::Rng;

#[derive(Deserialize)]
pub struct SignupPayload {
  email: String,
  password: String,
  first_name: String,
  last_name: String,
}

#[post("/signup")]
pub async fn signup(session: Session, signup_payload: Json<SignupPayload>) -> impl Responder {
  // check if user's email exists

  // validate email

  // validate password

  // validate name

  let key = rand::thread_rng().gen::<[u8; 32]>();
  session.set("sid", std::str::from_utf8(&key).unwrap().to_string());

  HttpResponse::Ok().body("hi")
}

use actix_web::web::ServiceConfig;
pub fn init_routes(config: &mut ServiceConfig) {
  config.service(signup);
}
