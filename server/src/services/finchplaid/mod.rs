use plaid::models::Account;
use std::collections::HashMap;

pub struct ApiClient {
  pub client_id: String,
  pub secret: String,
  pub client_name: String,
  pub configuration: plaid::apis::configuration::Configuration,
}

pub fn get_account_coefficients(accounts: &Vec<Account>) -> HashMap<String, f64> {
  accounts
    .iter()
    .map(|account: &Account| {
      (
        account.account_id.clone(),
        match account._type.as_str() {
          "depository" => 1.0,
          "credit" => -1.0,
          "loan" => 0.0,
          "investment" => 1.0,
          _ => 0.0,
        },
      )
    })
    .collect()
}
