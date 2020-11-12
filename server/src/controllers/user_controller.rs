pub use crate::models::{session_model, user_model};

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
  db: Data<mongodb::sync::Database>,
) -> HttpResponse {
  let res = user_model::User::new_from_signup(signup_payload.into_inner(), db.collection("Users"))
    .and_then(|user| {
      let _ = session_model::Session::new_user_session(
        db.collection("Sessions"),
        user._id.clone(),
        &session,
      );

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
  db: Data<mongodb::sync::Database>,
) -> HttpResponse {
  let res = user_model::User::new_from_login(login_payload.into_inner(), db.collection("Users"))
    .and_then(|user| {
      let _ = session_model::Session::new_user_session(
        db.collection("Sessions"),
        user._id.clone(),
        &session,
      );

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
