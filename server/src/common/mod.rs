pub mod errors;

mod money;
pub use money::*;

use actix_web::HttpResponse;
use serde::Serialize;

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

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use validator::ValidationError;
pub fn decimal_at_least_zero(d: &Decimal) -> Result<(), ValidationError> {
  match *d >= dec!(0) {
    true => Ok(()),
    false => Err(ValidationError::new("Field must be at least 0")),
  }
}

pub fn decimal_between_zero_or_hundred(d: &Decimal) -> Result<(), ValidationError> {
  match *d >= dec!(0) && *d <= dec!(100) {
    true => Ok(()),
    false => Err(ValidationError::new("Field must be between 0 and 100")),
  }
}
