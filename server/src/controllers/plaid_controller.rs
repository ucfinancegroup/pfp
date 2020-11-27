use crate::common::{errors::ApiError, into_response};
use crate::models::{session_model, user_model};
use crate::services::finchplaid::{ApiClient, ItemIdResponse, PublicTokenExchangeRequest};
use std::sync::{Arc, Mutex};

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
  plaid_client: Data<Arc<Mutex<ApiClient>>>,
  db: Data<Database>,
) -> HttpResponse {
  let pc = plaid_client.lock().unwrap();
  let config = &(pc.configuration);
  match session_model::Session::get_valid_session(&session, db.collection("Sessions")) {
    Err(e) => e.into(),
    Ok(finch_session) => {
      match plaid::apis::link_tokens_api::create_link_token(
        config,
        plaid::models::CreateLinkTokenRequest::new(
          pc.client_id.clone(),
          pc.secret.clone(),
          pc.client_name.clone(),
          vec!["US".to_string()],
          "en".to_string(),
          plaid::models::User::new(finch_session.user_id.to_string()),
          vec!["auth".to_string(), "transactions".to_string()],
        ),
      )
      .await
      {
        Ok(e) => into_response(e),
        Err(e) => ApiError::new(500, format!("{}", e)).into(),
      }
    }
  }
}

#[post("/plaid/public_token_exchange")]
async fn access_token(
  session: Session,
  plaid_client: Data<Arc<Mutex<ApiClient>>>,
  payload: Json<PublicTokenExchangeRequest>,
  db: Data<Database>,
) -> HttpResponse {
  let pc = plaid_client.lock().unwrap();
  let config = &(pc.configuration);
  let res = match session_model::Session::get_valid_session(&session, db.collection("Sessions")) {
    Err(e) => Err(e),
    Ok(finch_session) => plaid::apis::item_creation_api::exchange_token(
      config,
      plaid::models::ExchangeTokenRequest::new(
        pc.client_id.clone(),
        pc.secret.clone(),
        payload.into_inner().public_token,
      ),
    )
    .await
    .map_err(|_| ApiError::new(500, "".to_string()))
    .and_then(
      |plaid::models::ExchangeTokenResponse {
         access_token,
         item_id,
         request_id: _,
       }| {
        user_model::User::new_from_session(finch_session, db.collection("Users"))
          .and_then(|user| {
            user.add_new_account(access_token, item_id.clone(), db.collection("Users"))
          })
          .and_then(|_| Ok(ItemIdResponse { item_id: item_id }))
      },
    ),
  };

  match res {
    Ok(e) => into_response(e),
    Err(e) => e.into(),
  }
}

pub fn init_routes(config: &mut actix_web::web::ServiceConfig) {
  config.service(link_token);
  config.service(access_token);
}
