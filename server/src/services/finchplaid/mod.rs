pub struct ApiClient {
  pub client_id: String,
  pub secret: String,
  pub client_name: String,
  pub configuration: plaid::apis::configuration::Configuration,
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PublicTokenExchangeRequest {
  pub public_token: String,
}

#[derive(Serialize, Deserialize)]
pub struct ItemIdResponse {
  pub item_id: String,
}
