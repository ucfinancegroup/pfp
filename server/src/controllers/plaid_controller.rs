use crate::common::errors::ApiError;
use crate::models::user_model::{PlaidItem, User};
use crate::services::finchplaid::ApiClient;
use crate::services::users::UserService;
use actix_web::{
  post, get, put, delete,
  web::{Data, Path},
  HttpResponse,
};
use actix_web_validator::{Json, Validate};
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

// probably not right fields
#[derive(Validate, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountNewPayload {
  pub item_id: String,
  pub access_token: String,
}

#[post("/plaid/link_token")]
async fn link_token(plaid_client: Data<Arc<Mutex<ApiClient>>>, user: User) -> HttpResponse {
  let pc = plaid_client.lock().unwrap();
  let config = &(pc.configuration);

  crate::common::into_response_res(
    plaid::apis::link_tokens_api::create_link_token(
      config,
      plaid::models::CreateLinkTokenRequest::new(
        pc.client_id.clone(),
        pc.secret.clone(),
        pc.client_name.clone(),
        vec!["US".to_string()],
        "en".to_string(),
        plaid::models::User::new(user.id.unwrap().to_hex()),
        vec!["auth".to_string(), "transactions".to_string()],
      ),
    )
    .await
    .or(Err(ApiError::new(500, "".to_string()))),
  )
}

#[post("/plaid/public_token_exchange")]
async fn access_token(
  plaid_client: Data<Arc<Mutex<ApiClient>>>,
  payload: actix_web::web::Json<PublicTokenExchangeRequest>,
  user: User,
  user_service: Data<UserService>,
) -> HttpResponse {
  let res = help_access_token(
    payload.into_inner().public_token,
    plaid_client,
    user,
    user_service,
  )
  .await;

  crate::common::into_response_res(res)
}

async fn help_access_token(
  public_token: String,
  plaid_client: Data<Arc<Mutex<ApiClient>>>,
  user: User,
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

  user_service
    .add_new_account(user, item_access_token, item_id.clone())
    .await
    .and_then(|_| Ok(ItemIdResponse { item_id: item_id }))
}

#[get("/plaid/accounts")]
pub async fn get_accounts(
    user: User,
    user_service: Data<UserService>,
) -> HttpResponse {
    crate::common::into_response_res(user_service.get_accounts(user).await)
}

#[delete("plaid/accounts/{id}")]
pub async fn delete_accounts(
  Path(accounts_id): Path<String>,
  user: User,
  user_service: Data<UserService>,
) -> HttpResponse {
  let res = user_service.delete_accounts(accounts_id.clone(), user)
    .await
    .and_then(|_| Ok(ItemIdResponse { item_id: accounts_id }));

  crate::common::into_response_res(res)
}

#[put("/plaid/accounts/{id}")]
pub async fn update_accounts(
    Path(accounts_id): Path<String>,
    payload: actix_web::web::Json<PlaidItem>,
    user: User,
    user_service: Data<UserService>,
) -> HttpResponse {
  let res = user_service.update_accounts(accounts_id.clone(), payload.into_inner(), user)
    .await
    .and_then(|_| Ok(ItemIdResponse { item_id: accounts_id }));

  crate::common::into_response_res(res)
}

pub fn init_routes(config: &mut actix_web::web::ServiceConfig) {
  config.service(link_token);
  config.service(access_token);
  config.service(get_accounts);
  config.service(update_accounts);
  config.service(delete_accounts);
}
