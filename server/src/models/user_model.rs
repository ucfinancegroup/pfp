use crate::common::{errors::ApiError, Money};
use argon2::{self, Config};
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
  pub snapshots: Vec<Snapshot>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PlaidItem {
  pub item_id: String,
  pub access_token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Snapshot {
  pub net_worth: Money,

  pub running_savings: Money,
  pub running_spending: Money,
  pub running_income: Money,

  pub snapshot_time: i64,
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

impl Default for Snapshot {
  fn default() -> Snapshot {
    Snapshot {
      net_worth: Money { amount: 0 },
      running_savings: Money { amount: 0 },
      running_spending: Money { amount: 0 },
      running_income: Money { amount: 0 },
      snapshot_time: 0,
    }
  }
}

#[allow(unused_imports)]
use chrono::TimeZone;
use wither::mongodb::bson::doc;
use wither::prelude::Migrating;

impl Migrating for User {
  // Define any migrations which your model needs in this method.
  // As this is an interval migration, it will deactivate itself after the given threshold
  // date, so you could leave it in your code for as long as you would like.
  fn migrations() -> Vec<Box<dyn wither::Migration>> {
    // -- EXAMPLE --
    vec![Box::new(wither::IntervalMigration {
      name: "add snapshots field".to_string(),
      // NOTE: use a logical time here. A day after your deployment date, or the like.
      threshold: chrono::Utc.ymd(2021, 5, 1).and_hms(0, 0, 0),
      filter: doc! {"snapshots": doc!{"$exists": false}},
      set: Some(
        doc! {"snapshots": wither::mongodb::bson::to_bson(&Vec::<Snapshot>::new()).unwrap()},
      ),
      unset: None,
    })]
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_password_hashing() {
    let hashed = User::hash_password("password".to_string()).unwrap();

    let user = User {
      id: None,
      email: "email".to_string(),
      password: hashed,
      first_name: "first_name".to_string(),
      last_name: "last_name".to_string(),
      income: 0.0,
      accounts: vec![],
      snapshots: vec![],
    };

    assert_eq!(Ok(true), user.compare_password("password".to_string()));
    assert_eq!(Ok(false), user.compare_password("bad password".to_string()));
  }
}
