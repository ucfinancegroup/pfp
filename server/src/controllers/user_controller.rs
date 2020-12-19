use crate::models::user_model;
use crate::services::{sessions::SessionService, users::UserService};

use actix_session::Session;
use actix_web::{
  post,
  web::{Data, Json},
  HttpResponse,
};

#[post("/signup")]
pub async fn signup(
  session: Session,
  signup_payload: Json<user_model::SignupPayload>,
  user_service: Data<UserService>,
  session_service: Data<SessionService>,
) -> HttpResponse {
  let res = user_service
    .signup(signup_payload.into_inner())
    .and_then(|user| {
      let _ = session_service.new_user_session(user._id.clone(), &session);
      Ok(user_model::SignupResponse::new(user))
    });

  match res {
    Ok(e) => e.into(),
    Err(e) => e.into(),
  }
}

#[post("/login")]
pub async fn login(
  session: Session,
  login_payload: Json<user_model::LoginPayload>,
  user_service: Data<UserService>,
  session_service: Data<SessionService>,
) -> HttpResponse {
  let res = user_service
    .login(login_payload.into_inner())
    .and_then(|user| {
      let _ = session_service.new_user_session(user._id.clone(), &session);
      Ok(user_model::LoginResponse::new(user))
    });

  match res {
    Ok(e) => e.into(),
    Err(e) => e.into(),
  }
}

// you add the services here.
use actix_web::web::ServiceConfig;
pub fn init_routes(config: &mut ServiceConfig) {
  config.service(signup);
  config.service(login);
}
