use crate::common::errors::ApiError;
use crate::controllers::plaid_controller::AccountSuccess;
use crate::models::user_model::PlaidItem;
use actix_web::web::Data;
use plaid::models::{
  Account, RetrieveAnItemsAccountsRequest, RetrieveAnItemsAccountsResponse, RetrieveItemRequest,
  RetrieveItemResponse, SearchInstitutionbyIdRequest, SearchInstitutionbyIdResponse,
};
use serde_json::json;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct ApiClient {
  pub client_id: String,
  pub secret: String,
  pub client_name: String,
  pub configuration: plaid::apis::configuration::Configuration,
}

pub fn get_account_balance_coefficients(accounts: &Vec<Account>) -> HashMap<String, f64> {
  accounts
    .iter()
    .map(|account: &Account| {
      (
        account.account_id.clone(),
        match account._type.as_str() {
          "depository" => 1.0,
          "credit" => -1.0,
          "loan" => -1.0,
          "investment" => 1.0,
          _ => 0.0,
        },
      )
    })
    .collect()
}

pub fn get_account_transaction_coefficients(accounts: &Vec<Account>) -> HashMap<String, f64> {
  accounts
    .iter()
    .map(|account: &Account| {
      (
        account.account_id.clone(),
        match account._type.as_str() {
          "depository" => -1.0,
          "credit" => -1.0,
          "loan" => 0.0,
          "investment" => 1.0,
          _ => 0.0,
        },
      )
    })
    .collect()
}

pub async fn get_account_data(
  item: &PlaidItem,
  plaid_client: Data<Arc<Mutex<ApiClient>>>,
) -> Result<AccountSuccess, ApiError> {
  let accounts = get_item_accounts_for_new_snapshot(item, plaid_client.clone())
    .await?
    .accounts;

  let balance = calculate_net_worth(&accounts);
  let res = match get_institution_name(item, plaid_client.clone()).await {
    Ok(name) => AccountSuccess {
      item_id: "smaint".to_string(),
      balance: balance * 100.0,
      name: name,
    },
    Err(e) => return Err(e),
  };

  Ok(res)
}

async fn get_institution_name(
  item: &PlaidItem,
  plaid_client: Data<Arc<Mutex<ApiClient>>>,
) -> Result<String, ApiError> {
  let institution_id = get_item_data(item, plaid_client.clone())
    .await?
    .item
    .institution_id;
  println!("{}", institution_id);
  let name = get_institution_data(plaid_client, institution_id.clone())
    .await?
    .institution
    .name;
  println!("{}", name);
  Ok(name)
}

async fn get_item_data(
  item: &PlaidItem,
  plaid_client: Data<Arc<Mutex<ApiClient>>>,
) -> Result<RetrieveItemResponse, ApiError> {
  let pc = plaid_client.lock().unwrap();
  let config = &(pc.configuration);

  plaid::apis::item_management_api::retrieve_item(
    &config,
    RetrieveItemRequest::new(
      pc.client_id.clone(),
      pc.secret.clone(),
      item.access_token.clone(),
    ),
  )
  .await
  .map_err(|_| ApiError::new(500, "Error while getting plaid data".to_string()))
}

async fn get_institution_data(
  plaid_client: Data<Arc<Mutex<ApiClient>>>,
  institution_id: String,
) -> Result<SearchInstitutionbyIdResponse, ApiError> {
  let pc = plaid_client.lock().unwrap();
  let config = &(pc.configuration);

  let mut country_codes = Vec::new();
  country_codes.push("US".to_string());

  res = plaid::apis::institutions_api::search_institutionby_id(
    &config,
    SearchInstitutionbyIdRequest::new(
      institution_id,
      country_codes,
      pc.client_id.clone(),
      pc.secret.clone(),
    ),
  )
  .await;
  .map_err(|_| ApiError::new(500, "Error while getting bank data".to_string()))
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

async fn get_item_accounts_for_new_snapshot(
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
