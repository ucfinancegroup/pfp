use crate::common::{errors::ApiError, into_response};
use crate::services::finchplaid::ApiClient;
use crate::services::sessions::SessionService;
use crate::services::users::UserService;
use actix_session::Session;
use actix_web::{
  post,
  web::{Data, Json},
  HttpResponse,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Deserialize)]
pub struct PublicTokenExchangeRequest {
  pub public_token: String,
}

#[derive(Serialize)]
pub struct ItemIdResponse {
  pub item_id: String,
}

#[post("/plaid/link_token")]
async fn link_token(
  session: Session,
  plaid_client: Data<Arc<Mutex<ApiClient>>>,
  session_service: Data<SessionService>,
) -> HttpResponse {
  let pc = plaid_client.lock().unwrap();
  let config = &(pc.configuration);

  match session_service.get_valid_session(&session).await {
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
  session_service: Data<SessionService>,
  user_service: Data<UserService>,
) -> HttpResponse {
  let res = match session_service.get_valid_session(&session).await {
    Err(e) => Err(e),
    Ok(finch_session) => {
      help_access_token(
        finch_session,
        payload.into_inner().public_token,
        plaid_client,
        user_service,
      )
      .await
    }
  };

  crate::common::into_response_res(res)
}

async fn help_access_token(
  finch_session: crate::models::session_model::Session,
  public_token: String,
  plaid_client: Data<Arc<Mutex<ApiClient>>>,
  user_service: Data<UserService>,
) -> Result<ItemIdResponse, ApiError> {
  let pc = plaid_client.lock().unwrap();
  let config = &(pc.configuration);

  let exchanged = plaid::apis::item_creation_api::exchange_token(
    config,
    plaid::models::ExchangeTokenRequest::new(pc.client_id.clone(), pc.secret.clone(), public_token),
  )
  .await
  .map_err(|_| ApiError::new(500, "Plaid Client Error".to_string()))?;

  use plaid::models::ExchangeTokenResponse;

  let ExchangeTokenResponse {
    access_token: item_access_token,
    item_id,
    request_id: _,
  } = exchanged;

  let user = user_service.new_from_session(finch_session).await?;

  user_service
    .add_new_account(user, item_access_token, item_id.clone())
    .await
    .and_then(|_| Ok(ItemIdResponse { item_id: item_id }))
}

pub fn init_routes(config: &mut actix_web::web::ServiceConfig) {
  config.service(link_token);
  config.service(access_token);
}
