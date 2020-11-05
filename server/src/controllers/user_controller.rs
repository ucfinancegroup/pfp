pub use crate::models::user_model::*;

use serde::{Deserialize};
use actix_web::{Responder, HttpResponse, HttpRequest, web::Json, get, post, web::Data};
use actix_session::Session;

use rand::Rng;

use argon2::{self, Config};

#[post("/signup")]
pub async fn signup(session: Session, signup_payload: Json<SignupPayload>) -> impl Responder {
  if let Err(error_msg) = signup_payload.validate() {
    return HttpResponse::BadRequest().json(error_msg);
  }

  // check if user's email exists!!


  let key = rand::thread_rng().gen::<[u8; 32]>();
  session.set("sid", std::str::from_utf8(&key).unwrap().to_string());

  HttpResponse::Ok().json("Success")
}

#[get("/logout")]
pub async fn logout(session: Session) -> impl Responder {
  session.set("sid", "");
  HttpResponse::Ok().json("Success")
}

use actix_web::web::ServiceConfig;
pub fn init_routes(config: &mut ServiceConfig) {
  config.service(signup);
  config.service(logout);
}
