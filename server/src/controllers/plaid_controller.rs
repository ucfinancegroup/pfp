use crate::common::errors::ApiError;
use crate::models::user_model::User;
use crate::services::finchplaid;
use crate::services::users::UserService;
use actix_web::{
  delete, get, post,
  web::{Data, Path},
  HttpResponse,
};
use finchplaid::ApiClient;
use rust_decimal::Decimal;
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

#[derive(Serialize)]
pub struct AccountResponse {
  pub accounts: Vec<AccountSuccess>,
  #[serde(rename = "errors")]
  pub account_errors: Vec<AccountError>,
}

#[derive(Serialize)]
pub struct AccountSuccess {
  pub item_id: String,
  pub name: String,
  pub balance: Decimal,
}

#[derive(Serialize)]
pub struct AccountError {
  pub item_id: String,
  pub code: u16,
  pub message: String,
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
  let res: Result<ItemIdResponse, ApiError> = finchplaid::exchange_public_token_for_access_token(
    payload.into_inner().public_token,
    plaid_client,
    user,
    user_service,
  )
  .await;

  crate::common::into_response_res(res)
}

#[get("/plaid/accounts")]
pub async fn get_accounts(
  user: User,
  user_service: Data<UserService>,
  plaid_client: Data<Arc<Mutex<ApiClient>>>,
) -> HttpResponse {
  crate::common::into_response_res(user_service.get_accounts(user, plaid_client).await)
}

#[delete("plaid/accounts/{id}")]
pub async fn delete_account(
  Path(accounts_id): Path<String>,
  user: User,
  user_service: Data<UserService>,
) -> HttpResponse {
  let res = user_service
    .delete_account(accounts_id.clone(), user)
    .await
    .and_then(|_| {
      Ok(ItemIdResponse {
        item_id: accounts_id,
      })
    });

  crate::common::into_response_res(res)
}

pub fn init_routes(config: &mut actix_web::web::ServiceConfig) {
  config.service(link_token);
  config.service(access_token);
  config.service(get_accounts);
  config.service(delete_account);
}
