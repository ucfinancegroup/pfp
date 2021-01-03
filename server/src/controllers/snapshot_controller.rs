use crate::models::user_model::User;
use crate::services::{finchplaid::ApiClient, sessions::SessionService, users::UserService};
use actix_session::Session;
use actix_web::{get, web::Data, HttpResponse};
use std::sync::{Arc, Mutex};

#[get("/snapshots")]
pub async fn get_snapshots(
  session: Session,
  user_service: Data<UserService>,
  session_service: Data<SessionService>,
  plaid_client: Data<Arc<Mutex<ApiClient>>>,
) -> HttpResponse {
  crate::common::into_response_res(match session_service.get_valid_session(&session).await {
    Err(e) => Err(e),
    Ok(finch_session) => {
      let mut user: User = user_service.new_from_session(finch_session).await.unwrap();
      user_service.get_snapshots(&mut user, plaid_client).await
    }
  })
}

// you add the services here.
use actix_web::web::ServiceConfig;
pub fn init_routes(config: &mut ServiceConfig) {
  config.service(get_snapshots);
}
