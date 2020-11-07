use rand::Rng;
use serde::{Deserialize, Serialize};

use argon2::{self, Config};

// use bson::{oid::ObjectId};

use crate::common::errors::ApiError;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
  email: String,
  password: String,
  first_name: String,
  last_name: String,
  income: f64,
  // recurrings: Vec<ObjectId>,
  // snapshots: Vec<ObjectId>,
  // accounts: Vec<ObjectId>,
}

impl User {
  pub fn new_from_signup(data: SignupPayload) -> Result<User, ApiError> {
    let validated_signup_payload = data.validate();
    if let Err(e) = validated_signup_payload {
      return Err(ApiError::new(400, e));
    }

    // check for unused email

    if let Ok(password_hash) = User::hash_password(data.password) {
      Ok(User {
        email: data.email,
        password: password_hash,
        first_name: data.first_name,
        last_name: data.last_name,
        income: data.income,
      })
    } else {
      Err(ApiError::new(400, "Password hashing failed".to_string()))
    }
  }

  fn hash_password(plaintext: String) -> Result<String, ApiError> {
    let salt: [u8; 32] = rand::thread_rng().gen::<[u8; 32]>();
    let config = Config::default();
    argon2::hash_encoded(plaintext.as_bytes(), &salt, &config).map_err(|e| {
      ApiError::new(
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
        "Password hashing failed".to_string(),
      )
    })
  }

  fn compare_password(&self, plaintext: String) -> Result<bool, ApiError> {
    argon2::verify_encoded(&self.password.as_str(), plaintext.as_bytes()).map_err(|e| {
      ApiError::new(
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
        "Password checking failed".to_string(),
      )
    })
  }
}

#[derive(Deserialize)]
pub struct SignupPayload {
  pub email: String,
  pub password: String,
  pub first_name: String,
  pub last_name: String,
  pub income: f64,
}

pub trait Validation {
  fn validate(&self) -> Result<(), String>;
}

impl Validation for SignupPayload {
  fn validate(&self) -> Result<(), String> {
    return Ok(());
  }
}

#[derive(Serialize)]
pub struct SignupResponse {
  pub email: String,
  pub first_name: String,
  pub last_name: String,
  pub income: f64,
}

impl SignupResponse {
  pub fn new(u: User) -> SignupResponse {
    SignupResponse {
      email: u.email,
      first_name: u.first_name,
      last_name: u.last_name,
      income: u.income,
    }
  }
}

use actix_web::HttpResponse;
impl Into<HttpResponse> for SignupResponse {
  fn into(self) -> HttpResponse {
    HttpResponse::Ok().json(self)
  }
}
