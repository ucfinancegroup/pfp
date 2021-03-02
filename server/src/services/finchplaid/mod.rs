use crate::common::{errors::ApiError, Money};
use crate::controllers::plaid_controller::AccountSuccess;
use crate::controllers::plaid_controller::ItemIdResponse;
use crate::models::user_model::{PlaidItem, User};
use crate::services::{financial_products::FinProductService, users::UserService};
use actix_web::web::Data;
use plaid::models::{Account, RetrieveAnItemsAccountsRequest, RetrieveAnItemsAccountsResponse};
use rust_decimal::Decimal;
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::sync::{Arc, Mutex};

pub struct ApiClient {
  pub client_id: String,
  pub secret: String,
  pub client_name: String,
  pub configuration: plaid::apis::configuration::Configuration,
}

// (23 Feb 2021) -- only use positive-valued accounts
pub fn get_account_balance_coefficients(accounts: &Vec<Account>) -> HashMap<String, i64> {
  accounts
    .iter()
    .map(|account: &Account| {
      (
        account.account_id.clone(),
        match account._type.as_str() {
          "depository" => 1,
          "credit" => 0,
          "loan" => 0,
          "investment" => 1,
          _ => 0,
        },
      )
    })
    .collect()
}

pub fn get_account_transaction_coefficients(accounts: &Vec<Account>) -> HashMap<String, i64> {
  accounts
    .iter()
    .map(|account: &Account| {
      (
        account.account_id.clone(),
        match account._type.as_str() {
          "depository" => -1,
          "credit" => -1,
          "loan" => 0,
          "investment" => 1,
          _ => 0,
        },
      )
    })
    .collect()
}

pub async fn get_account_data<'a>(
  item: &PlaidItem,
  plaid_client: Data<Arc<Mutex<ApiClient>>>,
) -> Result<Vec<AccountSuccess>, ApiError> {
  let accounts = get_item_accounts(item, plaid_client.clone())
    .await?
    .accounts;
  let account_id_to_coeff = get_account_balance_coefficients(&accounts);

  let mut account_successes = Vec::new();

  for account in accounts.iter() {
    account_successes.push(AccountSuccess {
      item_id: item.item_id.clone(),
      balance: Decimal::try_from(account.balances.current)
        .map_err(|_| ApiError::new(500, "Decimal conversion error".to_string()))?
        * Decimal::new(
          *account_id_to_coeff
            .get(&account.account_id)
            .or(Some(&0))
            .unwrap(),
          0,
        ),
      name: account.name.clone(),
      account_type: account._type.to_string(),
      account_id: account.account_id.clone(),
    });
  }

  Ok(account_successes)
}

pub async fn get_net_worth(
  item: &PlaidItem,
  plaid_client: Data<Arc<Mutex<ApiClient>>>,
  excluded_accounts: &HashSet<String>,
) -> Result<Money, ApiError> {
  let accounts = get_item_accounts(item, plaid_client).await?.accounts;

  Ok(calculate_net_worth(&accounts, excluded_accounts))
}

pub fn calculate_net_worth(accounts: &Vec<Account>, excluded_accounts: &HashSet<String>) -> Money {
  // map each account to a coefficient for each transaction.
  let account_id_to_coeff = get_account_balance_coefficients(&accounts);

  //  calculate "net worth" of the item's accounts.
  accounts
    .iter()
    .filter(|&account: &&Account| !excluded_accounts.contains(&account.account_id))
    .fold(Money::new(0), |net, account: &Account| {
      let contribution = Money::new(Decimal::try_from(account.balances.current).unwrap())
        * *account_id_to_coeff
          .get(&account.account_id)
          .or(Some(&0))
          .unwrap();
      net + contribution
    })
}

pub async fn get_item_accounts(
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

pub async fn exchange_public_token_for_access_token(
  public_token: String,
  plaid_client: Data<Arc<Mutex<ApiClient>>>,
  user: User,
  user_service: Data<UserService>,
  fin_product_service: Data<FinProductService>,
) -> Result<ItemIdResponse, ApiError> {
  let exchanged = {
    let pc = plaid_client.lock().unwrap();
    let config = &(pc.configuration);
    plaid::apis::item_creation_api::exchange_token(
      config,
      plaid::models::ExchangeTokenRequest::new(
        pc.client_id.clone(),
        pc.secret.clone(),
        public_token,
      ),
    )
    .await
    .map_err(|_| ApiError::new(500, "Plaid Client Error".to_string()))
  }?;

  use plaid::models::ExchangeTokenResponse;

  let ExchangeTokenResponse {
    access_token: item_access_token,
    item_id,
    request_id: _,
  } = exchanged;

  user_service
    .add_new_account(
      user,
      item_access_token,
      item_id.clone(),
      plaid_client.clone(),
      fin_product_service,
    )
    .await
    .and_then(|_| Ok(ItemIdResponse { item_id: item_id }))
}
