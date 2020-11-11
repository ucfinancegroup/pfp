pub use crate::models::{session_model, user_model::*};

use actix_session::Session;
use actix_web::{
  post,
  web::{Data, Json},
  HttpResponse,
};

#[post("/signup")]
pub async fn signup(
  session: Session,
  signup_payload: Json<SignupPayload>,
  db: Data<mongodb::sync::Database>,
) -> HttpResponse {
  let user_res = User::new_from_signup(signup_payload.into_inner(), db.collection("Users"));

  match user_res {
    Err(e) => e.into(),
    Ok(user) => {
      let _session_res = session_model::Session::new_user_session(
        db.collection("Sessions"),
        user._id.clone(),
        &session,
      );

      SignupResponse::new(user).into()
    }
  }
}

// #[post("/login")]
// pub async fn login(
//   session: Session,
//   login_payload: Json<LoginPayload>,
//   db: Data<mongodb::Database>,
// ) -> HttpResponse {
// }

// #[get("/logout")]
// pub async fn logout(session: Session, db: Data<mongodb::Database>) -> impl Responder {
// }

use actix_web::web::ServiceConfig;
pub fn init_routes(config: &mut ServiceConfig) {
  config.service(signup);
  // config.service(login);
  // config.service(logout);
}
