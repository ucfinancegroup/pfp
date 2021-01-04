pub mod errors;

use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Money {
  pub amount: i64,
}

impl From<f64> for Money {
  fn from(f: f64) -> Money {
    Money {
      amount: (f * 100.0).floor() as i64,
    }
  }
}

impl Into<f64> for Money {
  fn into(self) -> f64 {
    (self.amount as f64) / 100.0
  }
}

pub fn into_response<T>(m: T) -> HttpResponse
where
  T: Serialize,
{
  HttpResponse::Ok().json(m)
}

pub fn into_response_res<T>(m: Result<T, errors::ApiError>) -> HttpResponse
where
  T: Serialize,
{
  match m {
    Ok(success) => HttpResponse::Ok().json(success),
    Err(error) => error.into(),
  }
}

pub fn into_bson_document<T>(m: &T) -> wither::mongodb::bson::Document
where
  T: Serialize,
{
  wither::mongodb::bson::to_bson(&m)
    .unwrap()
    .as_document()
    .unwrap()
    .clone()
}
