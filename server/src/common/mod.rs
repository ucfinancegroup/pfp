pub mod errors;

pub trait Validation {
  fn validate(&self) -> Result<(), String>;
}

use actix_web::HttpResponse;
use serde::Serialize;

pub fn into_response<T>(m: T) -> HttpResponse
where
  T: Serialize,
{
  HttpResponse::Ok().json(m)
}
