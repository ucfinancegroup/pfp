pub mod errors;

use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};
use crate::models::{
  user_model::{PlaidItem},
};
use actix_web::web::Data;
use std::sync::{Arc, Mutex};
use crate::common::errors::ApiError;
use crate::services::finchplaid::ApiClient;
use plaid::models::{
  Account, RetrieveAnItemsAccountsRequest, RetrieveAnItemsAccountsResponse,
};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Money {
  pub amount: i64,
}

impl From<f64> for Money {
  fn from(f: f64) -> Money {
    Money {
      amount: (f * 100.0).floor() as i64,
    }
  }
}

impl Into<f64> for Money {
  fn into(self) -> f64 {
    (self.amount as f64) / 100.0
  }
}

pub fn into_response<T>(m: T) -> HttpResponse
where
  T: Serialize,
{
  HttpResponse::Ok().json(m)
}

pub fn into_response_res<T>(m: Result<T, errors::ApiError>) -> HttpResponse
where
  T: Serialize,
{
  match m {
    Ok(success) => HttpResponse::Ok().json(success),
    Err(error) => error.into(),
  }
}

pub fn into_bson_document<T>(m: &T) -> wither::mongodb::bson::Document
where
  T: Serialize,
{
  wither::mongodb::bson::to_bson(&m)
    .unwrap()
    .as_document()
    .unwrap()
    .clone()
}

pub async fn get_net_worth(
  item: &PlaidItem,
  plaid_client: Data<Arc<Mutex<ApiClient>>>,
) -> Result<f64, ApiError> {
  let accounts = get_item_accounts_for_new_snapshot(item, plaid_client)
    .await?
    .accounts;

  Ok(calculate_net_worth(&accounts))
}

pub fn calculate_net_worth(accounts: &Vec<Account>) -> f64 {
  // map each account to a coefficient for each transaction.
  let account_id_to_coeff =
    crate::services::finchplaid::get_account_balance_coefficients(&accounts);

  //  calculate "net worth" of the item's accounts.
  accounts.iter().fold(0.0, |net, account: &Account| {
    let contribution: f64 = (account.balances.current as f64)
      * *account_id_to_coeff
        .get(&account.account_id)
        .or(Some(&0.0))
        .unwrap();
    net + contribution
  })
}

pub async fn get_item_accounts_for_new_snapshot(
  item: &PlaidItem,
  plaid_client: Data<Arc<Mutex<ApiClient>>>,
) -> Result<RetrieveAnItemsAccountsResponse, ApiError> {
  let pc = plaid_client.lock().unwrap();
  let config = &(pc.configuration);

  plaid::apis::item_management_api::retrieve_an_items_accounts(
    &config,
    RetrieveAnItemsAccountsRequest::new(
      pc.client_id.clone(),
      pc.secret.clone(),
      item.access_token.clone(),
    ),
  )
  .await
  .map_err(|_| ApiError::new(500, "Error while getting accounts".to_string()))
}
