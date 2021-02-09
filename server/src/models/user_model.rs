use crate::common::{errors::ApiError, Money};
use crate::models::{
  goal_model::Goal, insight_model::Insight, plan_model::Plan, recurring_model::Recurring,
};
use crate::services::{sessions::SessionService, users::UserService};
use actix_session::Session;
use actix_web::{
  dev::Payload, error::ErrorServiceUnavailable, error::ErrorUnauthorized, web::Data, Error,
  FromRequest, HttpRequest,
};
use actix_web_validator::Validate;
use argon2::{self, Config};
use chrono::Utc;
use futures::future::Future;
use rand::Rng;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::pin::Pin;
use wither::Model;

#[derive(Model, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct User {
  #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
  pub id: Option<wither::mongodb::bson::oid::ObjectId>,
  pub email: String,
  pub password: String,
  pub first_name: String,
  pub last_name: String,
  pub income: Decimal,
  pub location: Location,
  pub accounts: Vec<PlaidItem>,
  pub snapshots: Vec<Snapshot>,
  pub recurrings: Vec<Recurring>,
  pub goals: Vec<Goal>,
  pub insights: Vec<Insight>,
  pub plans: Vec<Plans>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PlaidItem {
  pub item_id: String,
  pub access_token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Snapshot {
  pub net_worth: Money,

  pub running_savings: Money,
  pub running_spending: Money,
  pub running_income: Money,

  pub snapshot_time: i64,
}

impl Snapshot {
  pub fn new<T: Into<Money>>(
    net_worth: T,
    running_savings: T,
    running_spending: T,
    running_income: T,
  ) -> Self {
    Self::new_with_time(
      net_worth,
      running_savings,
      running_spending,
      running_income,
      Utc::now().timestamp(),
    )
  }

  pub fn new_with_time<T: Into<Money>>(
    net_worth: T,
    running_savings: T,
    running_spending: T,
    running_income: T,
    snapshot_time: i64,
  ) -> Self {
    Snapshot {
      net_worth: net_worth.into(),
      running_savings: running_savings.into(),
      running_spending: running_spending.into(),
      running_income: running_income.into(),
      snapshot_time,
    }
  }
}

#[derive(Validate, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Location {
  pub has_location: bool,
  #[validate(range(min = -90.0, max = 90.0))]
  pub lat: f64,
  #[validate(range(min = -180.0, max = 180.0))]
  pub lon: f64,
}

impl Default for Location {
  fn default() -> Location {
    Location {
      has_location: false,
      lat: 0.0,
      lon: 0.0,
    }
  }
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
      net_worth: Money::new(Decimal::new(0, 0)),
      running_savings: Money::new(Decimal::new(0, 0)),
      running_spending: Money::new(Decimal::new(0, 0)),
      running_income: Money::new(Decimal::new(0, 0)),
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
    vec![
      Box::new(wither::IntervalMigration {
        name: "add snapshots field".to_string(),
        // NOTE: use a logical time here. A day after your deployment date, or the like.
        threshold: chrono::Utc.ymd(2021, 5, 1).and_hms(0, 0, 0),
        filter: doc! {"snapshots": doc!{"$exists": false}},
        set: Some(
          doc! {"snapshots": wither::mongodb::bson::to_bson(&Vec::<Snapshot>::new()).unwrap()},
        ),
        unset: None,
      }),
      Box::new(wither::IntervalMigration {
        name: "add recurrings field".to_string(),
        // NOTE: use a logical time here. A day after your deployment date, or the like.
        threshold: chrono::Utc.ymd(2021, 5, 1).and_hms(0, 0, 0),
        filter: doc! {"recurrings": doc!{"$exists": false}},
        set: Some(
          doc! {"recurrings": wither::mongodb::bson::to_bson(&Vec::<Recurring>::new()).unwrap()},
        ),
        unset: None,
      }),
      Box::new(wither::IntervalMigration {
        name: "add goals field".to_string(),
        // NOTE: use a logical time here. A day after your deployment date, or the like.
        threshold: chrono::Utc.ymd(2021, 5, 1).and_hms(0, 0, 0),
        filter: doc! {"goals": doc!{"$exists": false}},
        set: Some(doc! {"goals": wither::mongodb::bson::to_bson(&Vec::<Goal>::new()).unwrap()}),
        unset: None,
      }),
      Box::new(wither::IntervalMigration {
        name: "add location field".to_string(),
        // NOTE: use a logical time here. A day after your deployment date, or the like.
        threshold: chrono::Utc.ymd(2021, 5, 1).and_hms(0, 0, 0),
        filter: doc! {"location": doc!{"$exists": false}},
        set: Some(doc! {"location": wither::mongodb::bson::to_bson(&Location::default()).unwrap()}),
        unset: None,
      }),
      Box::new(wither::IntervalMigration {
        name: "add insights field".to_string(),
        // NOTE: use a logical time here. A day after your deployment date, or the like.
        threshold: chrono::Utc.ymd(2021, 5, 1).and_hms(0, 0, 0),
        filter: doc! {"insights": doc!{"$exists": false}},
        set: Some(
          doc! {"insights": wither::mongodb::bson::to_bson(&Vec::<Insight>::new()).unwrap()},
        ),
        unset: None,
      }),
      Box::new(wither::IntervalMigration {
        name: "add plans field".to_string(),
        // NOTE: use a logical time here. A day after your deployment date, or the like.
        threshold: chrono::Utc.ymd(2021, 5, 1).and_hms(0, 0, 0),
        filter: doc! {"plans": doc!{"$exists": false}},
        set: Some(doc! {"plans": wither::mongodb::bson::to_bson(&Vec::<Plan>::new()).unwrap()}),
        unset: None,
      }),
    ]
  }
}

// https://stackoverflow.com/questions/62269278/how-can-i-make-protected-routes-in-actix-web
impl FromRequest for User {
  type Config = ();
  type Error = Error;
  type Future = Pin<Box<dyn Future<Output = Result<User, Error>>>>;

  fn from_request(req: &HttpRequest, pl: &mut Payload) -> Self::Future {
    let session = Session::from_request(req, pl);
    let session_service_opt = req.app_data::<Data<SessionService>>();
    let user_service_opt = req.app_data::<Data<UserService>>();

    if session_service_opt.is_none() || user_service_opt.is_none() {
      return Box::pin(async {
        Err(ErrorServiceUnavailable(
          "SessionService or UserService unavailable",
        ))
      });
    }

    let session_service = session_service_opt.unwrap().clone();
    let user_service = user_service_opt.unwrap().clone();

    Box::pin(async move {
      let finch_session = session_service
        .get_valid_session(&session.await?)
        .await
        .or(Err(ErrorUnauthorized("")))?;
      let user: User = user_service
        .new_from_session(finch_session)
        .await
        .or(Err(ErrorUnauthorized("")))?;

      Ok(user)
    })
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
      income: 0.into(),
      location: Location::default(),
      accounts: vec![],
      snapshots: vec![],
      recurrings: vec![],
      goals: vec![],
      insights: vec![],
    };

    assert_eq!(Ok(true), user.compare_password("password".to_string()));
    assert_eq!(Ok(false), user.compare_password("bad password".to_string()));
  }
}
