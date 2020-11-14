use crate::common::errors::ApiError;
use crate::models::{session_model, user_model};
use crate::services::plaid;

use actix_session::Session;
use actix_web::{post, web::Data, HttpResponse};
use mongodb::sync::Database;

#[post("/plaid/link_token")]
async fn link_token(
  session: Session,
  plaid_client: Data<plaid::Client>,
  db: Data<Database>,
) -> HttpResponse {
  let sesh = session_model::Session::new_from_store(&session);

  if sesh.is_none() {
    return ApiError::new(401, "No session".to_string()).into();
  }

  let finch_session = sesh.unwrap();

  if !finch_session.is_valid(db.collection("Sessions")) {
    return ApiError::new(401, "Invalid session".to_string()).into();
  }

  match plaid_client
    .link_token_create(plaid::LinkTokenCreateRequest::with_user_id(
      &finch_session.user_id.to_hex(),
    ))
    .await
  {
    Ok(e) => e.into(),
    Err(e) => e.into(),
  }
}

#[post("/plaid/access_token")]
async fn access_token() -> HttpResponse {
  HttpResponse::Ok().body("not impl")
}

pub fn init_routes(config: &mut actix_web::web::ServiceConfig) {
  config.service(link_token);
  config.service(access_token);
}
