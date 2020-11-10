pub use crate::models::user_model::*;

use actix_session::Session;
use actix_web::{
  get, post,
  web::{Data, Json},
  HttpResponse, Responder,
};

use rand::Rng;

use crate::common::errors::ApiError;

#[post("/signup")]
pub async fn signup(
  session: Session,
  signup_payload: Json<SignupPayload>,
  db: Data<mongodb::Database>,
) -> HttpResponse {
  // tell user model to do a signup
  // it'll return errors if there are any
  let res = User::new_from_signup(signup_payload.into_inner()).and_then(|user| {
    let key = rand::thread_rng().gen::<[u8; 32]>();
    match session.set("sid", std::str::from_utf8(&key).unwrap().to_string()) {
      Ok(_) => Ok(user),
      Err(_) => Err(ApiError::new(500, "Could not store session".to_string())),
    }
  });

  match res {
    Ok(user) => SignupResponse::new(user).into(),
    Err(e) => e.into(),
  }
}

// #[post("/login")]
// pub async fn login(
//   session: Session,
//   login_payload: Json<LoginPayload>,
//   db: Data<mongodb::Database>,
// ) -> HttpResponse {
// }

#[get("/logout")]
pub async fn logout(session: Session, db: Data<mongodb::Database>) -> impl Responder {
  let _ = session.set("sid", "");
  HttpResponse::Ok().json("Success")
}

use actix_web::web::ServiceConfig;
pub fn init_routes(config: &mut ServiceConfig) {
  config.service(signup);
  // config.service(login);
  config.service(logout);
}
