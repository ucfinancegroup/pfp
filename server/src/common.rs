pub mod errors {
  #[derive(serde::Serialize, Debug, Eq, PartialEq)]
  pub struct ApiError {
    code: u16,
    message: String,
  }

  impl ApiError {
    pub fn new(code: u16, message: String) -> ApiError {
      ApiError {
        code: code,
        message: message,
      }
    }
  }

  use actix_web::{http::StatusCode, HttpResponse};

  impl Into<HttpResponse> for ApiError {
    fn into(self) -> HttpResponse {
      if let Ok(code) = StatusCode::from_u16(self.code) {
        HttpResponse::build(code).json(self)
      } else {
        HttpResponse::InternalServerError().json(self)
      }
    }
  }
}

pub trait Validation {
  fn validate(&self) -> Result<(), String>;
}
