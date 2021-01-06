use crate::common::errors::ApiError;
use crate::models::user_model::User;
use crate::services::{sessions::SessionService, users::UserService};
use actix_session::Session;
use actix_web::{
  post,
  web::{Data},
  HttpResponse,
};
use actix_web_validator::Json;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Validate, Deserialize, PartialEq)]
pub struct SignupPayload {
  #[validate(email)]
  pub email: String,
  #[validate(length(min = 1))]
  pub password: String,
  #[validate(length(min = 1))]
  pub first_name: String,
  #[validate(length(min = 1))]
  pub last_name: String,
  #[validate(range(min = 0))]
  pub income: f64,
}

#[derive(Serialize, PartialEq)]
struct SignupResponse {
  pub email: String,
  pub first_name: String,
  pub last_name: String,
  pub income: f64,
}

impl SignupResponse {
  pub fn new(u: User) -> SignupResponse {
    SignupResponse {
      email: u.email,
      first_name: u.first_name,
      last_name: u.last_name,
      income: u.income,
    }
  }
}

#[derive(Validate, Deserialize, PartialEq)]
pub struct LoginPayload {
  #[validate(email)]
  pub email: String,
  #[validate(length(min = 1))]
  pub password: String,
}

type LoginResponse = SignupResponse;

#[post("/signup")]
pub async fn signup(
  session: Session,
  signup_payload: Json<SignupPayload>,
  user_service: Data<UserService>,
  session_service: Data<SessionService>,
) -> HttpResponse {

  let res = match user_service.signup(signup_payload.into_inner()).await {
    Ok(user) => session_service
      .new_user_session(&user, &session)
      .await
      .and_then(|_| Ok(SignupResponse::new(user))),
    Err(e) => Err(e),
  };

  crate::common::into_response_res(res)
}

#[post("/login")]
pub async fn login(
  session: Session,
  login_payload: Json<LoginPayload>,
  user_service: Data<UserService>,
  session_service: Data<SessionService>,
) -> HttpResponse {

  let res = match user_service.login(login_payload.into_inner()).await {
    Ok(user) => session_service
      .new_user_session(&user, &session)
      .await
      .and_then(|_| Ok(LoginResponse::new(user))),
    Err(e) => Err(e),
  };

  crate::common::into_response_res(res)
}

// you add the services here.
use actix_web::web::ServiceConfig;
pub fn init_routes(config: &mut ServiceConfig) {
  config.service(signup);
  config.service(login);
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_login_payload() {
    assert_eq!(
      Ok(()),
      LoginPayload {
        email: "me@chucknorris.com".to_string(),
        password: "password".to_string(),
      }
      .validate()
    );

    assert!(LoginPayload {
      email: "not an email".to_string(),
      password: "password".to_string(),
    }
    .validate()
    .is_err());

    assert!(LoginPayload {
      email: "me@chucknorris.com".to_string(),
      password: "".to_string(),
    }
    .validate()
    .is_err());
  }

  #[test]
  fn test_signup_payload() {
    assert_eq!(
      Ok(()),
      SignupPayload {
        email: "me@chucknorris.com".to_string(),
        password: "fafdfdf".to_string(),
        first_name: "first name".to_string(),
        last_name: "last name".to_string(),
        income: 1000 as f64
      }
      .validate()
    );

    // should fail on Negative Income
    assert!(SignupPayload {
      email: "me@chucknorris.com".to_string(),
      password: "fadfdfda".to_string(),
      first_name: "first name".to_string(),
      last_name: "last name".to_string(),
      income: -1 as f64
    }
    .validate()
    .is_err());

    // fail on bad email
    assert!(SignupPayload {
      email: "bad email".to_string(),
      password: "".to_string(),
      first_name: "first name".to_string(),
      last_name: "last name".to_string(),
      income: 1000 as f64
    }
    .validate()
    .is_err());

    // fail on empty password
    assert!(SignupPayload {
      email: "me@chucknorris.com".to_string(),
      password: "".to_string(),
      first_name: "first name".to_string(),
      last_name: "last name".to_string(),
      income: 1000 as f64
    }
    .validate()
    .is_err());

    // eventually fail on empty name
    assert!(SignupPayload {
      email: "me@chucknorris.com".to_string(),
      password: "fadfdf".to_string(),
      first_name: "".to_string(),
      last_name: "".to_string(),
      income: 1000 as f64
    }
    .validate()
    .is_err());
  }
}
