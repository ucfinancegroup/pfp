use crate::common::errors::ApiError;
use argon2::{self, Config};
// use mongodb::bson::{doc, oid::ObjectId};
use rand::Rng;
use serde::{Deserialize, Serialize};
use wither::Model;

#[derive(Model, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct User {
  #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
  pub id: Option<wither::mongodb::bson::oid::ObjectId>,
  pub email: String,
  pub password: String,
  pub first_name: String,
  pub last_name: String,
  pub income: f64,
  pub accounts: Vec<PlaidItem>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PlaidItem {
  pub item_id: String,
  pub access_token: String,
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

// impl<'a> wither::Model for User {
//   /// The name of this model's collection.
//   const COLLECTION_NAME: &'static str = "Users";

//   /// Implement the getter for the ID of a model instance.
//   fn id(&self) -> Option<wither::mongodb::bson::oid::ObjectId> {
//     return self.id.clone();
//   }

//   /// Implement the setter for the ID of a model instance.
//   fn set_id(&mut self, oid: wither::mongodb::bson::oid::ObjectId) {
//     self.id = Some(oid);
//   }
// }

#[cfg(test)]
mod test {
  use super::*;

  #[allow(non_snake_case)]
  #[allow(dead_code)]
  fn test_PasswordHashing() {
    let hashed = User::hash_password("password".to_string()).unwrap();

    let user = User {
      id: None,
      email: "email".to_string(),
      password: hashed,
      first_name: "first_name".to_string(),
      last_name: "last_name".to_string(),
      income: 0.0,
      accounts: vec![],
    };

    assert_eq!(Ok(true), user.compare_password("password".to_string()));
    assert_eq!(Ok(false), user.compare_password("bad password".to_string()));
  }
}
