pub struct ApiClient {
  pub client_id: String,
  pub secret: String,
  pub client_name: String,
  pub configuration: plaid::apis::configuration::Configuration,
}
