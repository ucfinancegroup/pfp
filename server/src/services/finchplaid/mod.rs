use crate::common::{errors::ApiError, Money};
use crate::controllers::plaid_controller::AccountSuccess;
use crate::controllers::plaid_controller::ItemIdResponse;
use crate::models::user_model::{PlaidItem, User};
use crate::services::{financial_products::FinProductService, users::UserService};
use actix_web::web::Data;
use plaid::apis::configuration::Configuration;
use plaid::models::*;
use rust_decimal::Decimal;
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;

#[derive(Clone)]
pub struct ApiClient {
  pub client_id: String,
  pub secret: String,
  pub client_name: String,
}

impl ApiClient {
  fn get_configuration() -> Configuration {
    Configuration::default()
  }

  pub async fn create_link_token(
    &self,
    user_id: String,
  ) -> Result<CreateLinkTokenResponse, ApiError> {
    let config = Self::get_configuration();
    plaid::apis::link_tokens_api::create_link_token(
      &config,
      plaid::models::CreateLinkTokenRequest::new(
        self.client_id.clone(),
        self.secret.clone(),
        self.client_name.clone(),
        vec!["US".to_string()],
        "en".to_string(),
        plaid::models::User::new(user_id),
        vec!["auth".to_string(), "transactions".to_string()],
      ),
    )
    .await
    .or(Err(ApiError::new(500, "".to_string())))
  }

  pub async fn retrieve_an_items_accounts(
    &self,
    item_access_token: String,
  ) -> Result<RetrieveAnItemsAccountsResponse, ApiError> {
    let config = Self::get_configuration();
    plaid::apis::item_management_api::retrieve_an_items_accounts(
      &config,
      RetrieveAnItemsAccountsRequest::new(
        self.client_id.clone(),
        self.secret.clone(),
        item_access_token,
      ),
    )
    .await
    .or(Err(ApiError::new(
      500,
      "Error while getting accounts".to_string(),
    )))
  }

  pub async fn exchange_token(
    &self,
    public_token: String,
  ) -> Result<ExchangeTokenResponse, ApiError> {
    let config = Self::get_configuration();

    plaid::apis::item_creation_api::exchange_token(
      &config,
      plaid::models::ExchangeTokenRequest::new(
        self.client_id.clone(),
        self.secret.clone(),
        public_token,
      ),
    )
    .await
    .map_err(|_| ApiError::new(500, "Plaid Exchange Token Error".to_string()))
  }

  pub async fn retrieve_transactions(
    &self,
    item_access_token: String,
    date: String,
  ) -> Result<RetrieveTransactionsResponse, ApiError> {
    let config = Self::get_configuration();
    plaid::apis::transactions_api::retrieve_transactions(
      &config,
      RetrieveTransactionsRequest::new(
        self.client_id.clone(),
        self.secret.clone(),
        item_access_token,
        date.clone(),
        date,
      ),
    )
    .await
    .map_err(|_| ApiError::new(500, "Error while getting transactions".to_string()))
  }
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
  plaid_client: Data<ApiClient>,
) -> Result<Vec<AccountSuccess>, ApiError> {
  let accounts = plaid_client
    .retrieve_an_items_accounts(item.access_token.clone())
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
  plaid_client: Data<ApiClient>,
  excluded_accounts: &HashSet<String>,
) -> Result<Money, ApiError> {
  let accounts = plaid_client
    .retrieve_an_items_accounts(item.access_token.clone())
    .await?
    .accounts;

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

pub async fn exchange_public_token_for_access_token(
  public_token: String,
  plaid_client: Data<ApiClient>,
  user: User,
  user_service: Data<UserService>,
  fin_product_service: Data<FinProductService>,
) -> Result<ItemIdResponse, ApiError> {
  let exchanged = plaid_client.exchange_token(public_token).await?;

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
