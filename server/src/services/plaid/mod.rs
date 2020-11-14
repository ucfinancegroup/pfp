use crate::common::errors::ApiError;
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserInfo {
  pub client_user_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct LinkTokenCreateRequest {
  pub user: UserInfo,
  pub client_id: Option<String>,
  pub secret: Option<String>,
  pub client_name: Option<String>,
  language: String,
  country_codes: Vec<String>,
  products: Vec<String>,
}

impl LinkTokenCreateRequest {
  pub fn with_user_id(user_id: &str) -> LinkTokenCreateRequest {
    LinkTokenCreateRequest {
      user: UserInfo {
        client_user_id: user_id.to_string(),
      },
      client_id: None,
      secret: None,
      client_name: None,
      language: "en".to_string(),
      country_codes: vec!["US".to_string()],
      products: vec!["transactions".to_string(), "auth".to_string()],
    }
  }
}

#[derive(Serialize, Deserialize)]
pub struct LinkTokenCreateResponse {
  expiration: String,
  link_token: String,
  request_id: Option<String>,
}

impl Into<HttpResponse> for LinkTokenCreateResponse {
  fn into(self) -> HttpResponse {
    HttpResponse::Ok().json(self)
  }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum Environment {
  Sandbox,
  Development,
  Production,
}

impl Environment {
  pub fn to_plaid_host(&self) -> String {
    let subdomain = match self {
      Environment::Sandbox => "sandbox",
      Environment::Development => "development",
      Environment::Production => "production",
    };
    format!("https://{}.plaid.com", subdomain).to_string()
  }
  pub fn to_plaid_host_with_endpoint(&self, endpoint: &str) -> String {
    format!("{}{}", self.to_plaid_host(), endpoint).to_string()
  }
}

#[derive(Clone, Debug)]
pub struct Client {
  pub client_id: String,
  pub secret: String,
  pub client_name: String,
  pub environment: Environment,
}

impl Client {
  pub async fn link_token_create(
    &self,
    mut options: LinkTokenCreateRequest,
  ) -> Result<LinkTokenCreateResponse, ApiError> {
    options.client_id = options.client_id.or(Some(self.client_id.clone()));
    options.secret = options.secret.or(Some(self.secret.clone()));
    options.client_name = options.client_name.or(Some(self.client_name.clone()));

    let client = reqwest::Client::new();
    let res = client
      .post(
        &self
          .environment
          .to_plaid_host_with_endpoint("/link/token/create"),
      )
      .json(&options)
      .send()
      .await;

    match res {
      Ok(r) => r
        .json::<LinkTokenCreateResponse>()
        .await
        .map_err(|_| ApiError::new(500, "Parse Error".to_string())),
      Err(_) => Err(ApiError::new(500, "Plaid Error".to_string())),
    }
  }
}
