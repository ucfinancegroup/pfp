#[derive(serde::Serialize, Debug, Eq, PartialEq, Clone)]
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

pub struct AppError {
  message: &'static str,
}

impl AppError {
  pub fn new(message: &'static str) -> AppError {
    AppError { message }
  }
}

impl Into<ApiError> for AppError {
  fn into(self) -> ApiError {
    ApiError::new(500, self.message.to_string())
  }
}
