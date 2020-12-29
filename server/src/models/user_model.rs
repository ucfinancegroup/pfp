use argon2::{self, Config};
use rand::Rng;
use serde::{Deserialize, Serialize};

use mongodb::bson::{doc, oid::ObjectId};

use crate::common::errors::ApiError;

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

#[cfg(test)]
mod test {
  use super::*;

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
