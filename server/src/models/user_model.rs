use argon2::{self, Config};
use rand::Rng;
use serde::{Deserialize, Serialize};

use mongodb::bson::{doc, oid::ObjectId};

use actix_web::HttpResponse;

use crate::common::{errors::ApiError, Validation};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PlaidItem {
  pub item_id: String,
  pub access_token: String,
}

impl PlaidItem {
  pub fn new(item_id: String, access_token: String) -> PlaidItem {
    PlaidItem {
      item_id: item_id,
      access_token: access_token,
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct User {
  pub _id: ObjectId,
  pub email: String,
  pub password: String,
  pub first_name: String,
  pub last_name: String,
  pub income: f64,
  // recurrings: Vec<ObjectId>,
  // snapshots: Vec<ObjectId>,
  pub accounts: Option<Vec<PlaidItem>>,
}

impl User {
  pub fn hash_password(plaintext: String) -> Result<String, ApiError> {
    let salt: [u8; 32] = rand::thread_rng().gen::<[u8; 32]>();
    let config = Config::default();
    argon2::hash_encoded(plaintext.as_bytes(), &salt, &config).map_err(|_| {
      ApiError::new(
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
        "Password hashing failed".to_string(),
      )
    })
  }

  pub fn compare_password(&self, plaintext: String) -> Result<bool, ApiError> {
    argon2::verify_encoded(&self.password.as_str(), plaintext.as_bytes()).map_err(|_| {
      ApiError::new(
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
        "Password checking failed".to_string(),
      )
    })
  }
}

impl std::convert::From<User> for mongodb::bson::Bson {
  fn from(s: User) -> mongodb::bson::Bson {
    mongodb::bson::to_bson(&s).unwrap()
  }
}

#[derive(Deserialize, PartialEq)]
pub struct SignupPayload {
  pub email: String,
  pub password: String,
  pub first_name: String,
  pub last_name: String,
  pub income: f64,
}

impl Validation for SignupPayload {
  fn validate(&self) -> Result<(), String> {
    return Ok(());
  }
}

#[derive(Serialize, PartialEq)]
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

impl Into<HttpResponse> for SignupResponse {
  fn into(self) -> HttpResponse {
    HttpResponse::Ok().json(self)
  }
}

#[derive(Deserialize, PartialEq)]
pub struct LoginPayload {
  pub email: String,
  pub password: String,
}

impl Validation for LoginPayload {
  fn validate(&self) -> Result<(), String> {
    return Ok(());
  }
}

pub type LoginResponse = SignupResponse;

#[cfg(test)]
mod test {
  use super::*;

  #[allow(non_snake_case)]
  #[allow(dead_code)]
  fn test_LoginResponse() {}

  #[allow(non_snake_case)]
  #[allow(dead_code)]
  fn test_SignupResponse() {}

  #[allow(non_snake_case)]
  #[allow(dead_code)]
  fn test_LoginPayload() {
    assert_eq!(
      Ok(()),
      LoginPayload {
        email: "me@chucknorris.com".to_string(),
        password: "password".to_string(),
      }
      .validate()
    );

    // eventually should be not ok
    assert_eq!(
      Ok(()),
      LoginPayload {
        email: "not an email".to_string(),
        password: "password".to_string(),
      }
      .validate()
    );

    // eventually should be not ok
    assert_eq!(
      Ok(()),
      LoginPayload {
        email: "me@chucknorris.com".to_string(),
        password: "".to_string(),
      }
      .validate()
    );
  }

  #[allow(non_snake_case)]
  #[allow(dead_code)]
  fn test_SignupPayload() {
    assert_eq!(
      Ok(()),
      SignupPayload {
        email: "me@chucknorris.com".to_string(),
        password: "fafdfdf".to_string(),
        first_name: "first name".to_string(),
        last_name: "last name".to_string(),
        income: 1000 as f64
      }
      .validate()
    );

    // should eventually fail on Negative Income
    assert_eq!(
      Ok(()),
      SignupPayload {
        email: "me@chucknorris.com".to_string(),
        password: "fadfdfda".to_string(),
        first_name: "first name".to_string(),
        last_name: "last name".to_string(),
        income: -1 as f64
      }
      .validate()
    );

    // fail on bad email
    assert_eq!(
      Ok(()),
      SignupPayload {
        email: "bad email".to_string(),
        password: "".to_string(),
        first_name: "first name".to_string(),
        last_name: "last name".to_string(),
        income: 1000 as f64
      }
      .validate()
    );

    // eventually fail on empty password
    assert_eq!(
      Ok(()),
      SignupPayload {
        email: "me@chucknorris.com".to_string(),
        password: "".to_string(),
        first_name: "first name".to_string(),
        last_name: "last name".to_string(),
        income: 1000 as f64
      }
      .validate()
    );

    // eventually fail on empty name
    assert_eq!(
      Ok(()),
      SignupPayload {
        email: "me@chucknorris.com".to_string(),
        password: "fadfdf".to_string(),
        first_name: "".to_string(),
        last_name: "".to_string(),
        income: 1000 as f64
      }
      .validate()
    );
  }

  #[allow(non_snake_case)]
  #[allow(dead_code)]
  fn test_PasswordHashing() {
    let hashed = User::hash_password("password".to_string()).unwrap();

    let user = User {
      _id: ObjectId::new(),
      email: "email".to_string(),
      password: hashed,
      first_name: "first_name".to_string(),
      last_name: "last_name".to_string(),
      income: 0.0,
      accounts: None,
    };

    assert_eq!(Ok(true), user.compare_password("password".to_string()));
    assert_eq!(Ok(false), user.compare_password("bad password".to_string()));
  }
}
