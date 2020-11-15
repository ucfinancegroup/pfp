use crate::models::{session_model, user_model};
use crate::services::plaid;

use actix_session::Session;
use actix_web::{
  post,
  web::{Data, Json},
  HttpResponse,
};
use mongodb::sync::Database;

#[post("/plaid/link_token")]
async fn link_token(
  session: Session,
  plaid_client: Data<plaid::Client>,
  db: Data<Database>,
) -> HttpResponse {
  match session_model::Session::get_valid_session(&session, db.collection("Sessions")) {
    Err(e) => e.into(),
    Ok(finch_session) => {
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
  }
}

#[post("/plaid/public_token_exchange")]
async fn access_token(
  session: Session,
  plaid_client: Data<plaid::Client>,
  payload: Json<plaid::PublicTokenExchangeRequest>,
  db: Data<Database>,
) -> HttpResponse {
  let res = match session_model::Session::get_valid_session(&session, db.collection("Sessions")) {
    Err(e) => Err(e),
    Ok(finch_session) => plaid_client
      .public_token_exchange(payload.into_inner())
      .await
      .and_then(
        |plaid::PublicTokenExchangeResponse {
           access_token,
           item_id,
           request_id: _,
         }| {
          user_model::User::new_from_session(finch_session, db.collection("Users"))
            .and_then(|user| {
              user.add_new_account(access_token, item_id.clone(), db.collection("Users"))
            })
            .and_then(|_| Ok(plaid::ItemIdResponse { item_id: item_id }))
        },
      ),
  };

  match res {
    Ok(e) => e.into(),
    Err(e) => e.into(),
  }
}

pub fn init_routes(config: &mut actix_web::web::ServiceConfig) {
  config.service(link_token);
  config.service(access_token);
}
