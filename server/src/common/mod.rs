pub mod errors;
pub mod finchlog;
mod money;
use crate::models::plan_model::AllocationProportion;
use actix_web::HttpResponse;
use chrono::{Datelike, TimeZone, Utc};
pub use money::*;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::Serialize;
use validator::ValidationError;
use wither::{mongodb::bson::oid::ObjectId, Model};

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

pub fn ensure_id<T>(obj: &mut T)
where
  T: Model,
{
  if obj.id().is_none() {
    obj.set_id(ObjectId::new())
  }
}

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

pub fn allocation_schema_sum_around_100(
  schema: &Vec<AllocationProportion>,
) -> Result<(), ValidationError> {
  let total = schema
    .into_iter()
    .fold(dec!(0.0), |total, x| total + x.proportion);

  match total > dec!(98.0) && total < dec!(102.0) {
    true => Ok(()),
    false => Err(ValidationError::new(
      "Unable to properly calculate allocations",
    )),
  }
}

pub fn min_age_13yo(birthday: &String) -> Result<(), ValidationError> {
  log::debug!("Got birthday {} ", birthday);

  let date = chrono::NaiveDate::parse_from_str(birthday, "%Y-%m-%d")
    .map_err(|e| {
      log::warn!("{}", e);
      ValidationError::new("invalid birthday format. Expected YYYY-mm-dd")
    })
    .and_then(|date| Ok(Utc.from_local_datetime(&date.and_hms(0, 0, 0))))?
    .earliest()
    .ok_or(ValidationError::new(
      "Could not extract valid UTC dateTime from birthday",
    ))?;

  let mut thirteen_years_ago = Utc::now();
  thirteen_years_ago = thirteen_years_ago
    .with_year(thirteen_years_ago.year() - 13)
    .unwrap();

  log::debug!("13y ago {}, parsed {}", thirteen_years_ago, date);

  match date < thirteen_years_ago {
    true => Ok(()),
    false => Err(ValidationError::new("Must be at least 13yo to sign up")),
  }
}

// trait for models that should have examples
pub trait Examples {
  type Output;
  fn examples() -> Vec<Self::Output>;
}
