use crate::models::user_model::{Location, User};
use crate::services::{sessions::SessionService, users::UserService};
use actix_session::Session;
use actix_web::{post, put, web::Data, HttpResponse};
use actix_web_validator::{Json, Validate};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use validator::ValidationError;

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
  #[validate(custom = "crate::common::decimal_at_least_zero")]
  pub income: Decimal,
  pub location: Location,
  #[validate(custom = "crate::common::min_age_13yo")]
  pub birthday: String,
}

#[derive(Serialize, PartialEq)]
struct SignupResponse {
  pub email: String,
  pub first_name: String,
  pub last_name: String,
  pub income: Decimal,
  pub location: Location,
}

impl SignupResponse {
  pub fn new(u: User) -> SignupResponse {
    SignupResponse {
      email: u.email,
      first_name: u.first_name,
      last_name: u.last_name,
      income: u.income,
      location: u.location,
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

#[derive(Validate, Deserialize, PartialEq)]
pub struct UpdatePayload {
  #[validate(email)]
  pub email: Option<String>,
  #[validate(length(min = 1))]
  pub password: Option<String>,
  #[validate(length(min = 1))]
  pub first_name: Option<String>,
  #[validate(length(min = 1))]
  pub last_name: Option<String>,
  #[validate(custom = "crate::common::decimal_at_least_zero")]
  pub income: Option<Decimal>,
  pub location: Option<Location>,
  #[validate(custom = "crate::common::min_age_13yo")]
  pub birthday: Option<String>,
}

type UpdateResponse = SignupResponse;

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

#[put("/update/user")]
pub async fn update_user(
  user: User,
  update_payload: Json<UpdatePayload>,
  user_service: Data<UserService>,
) -> HttpResponse {
  let res = user_service
    .update(user, update_payload.into_inner())
    .await
    .and_then(|updated| Ok(UpdateResponse::new(updated)));

  crate::common::into_response_res(res)
}

#[post("/logout")]
pub async fn logout(session: Session, session_service: Data<SessionService>) -> HttpResponse {
  let res = match session_service.get_valid_session(&session).await {
    Ok(finch_session) => {
      session_service.invalidate(&finch_session).await;
      Ok("Logged Out")
    }
    Err(e) => Err(e),
  };

  crate::common::into_response_res(res)
}

#[derive(Deserialize, Validate)]
#[validate(schema(function = "validate_validate_user_payload"))]
pub struct ValidateUserPayload {
  pub field: ValidateUserContentType,
  pub content: String,
}

#[derive(Deserialize, PartialEq)]
pub enum ValidateUserContentType {
  Email,
  Password,
  Birthday,
}

pub fn validate_validate_user_payload(
  payload: &ValidateUserPayload,
) -> Result<(), ValidationError> {
  use ValidateUserContentType::*;
  match payload.field {
    Email => {
      if validator::validate_email(payload.content.clone()) {
        Ok(())
      } else {
        Err(ValidationError::new("Email is invaid"))
      }
    }
    Password => {
      if validator::validate_length(payload.content.clone(), Some(8), None, None) {
        Ok(())
      } else {
        Err(ValidationError::new(
          "Password must be at least 8 characters long",
        ))
      }
    }
    Birthday => crate::common::min_age_13yo(&payload.content),
  }
}

#[post("/validate/user")]
pub async fn validate_user(
  payload: Json<ValidateUserPayload>,
  user_service: Data<UserService>,
) -> HttpResponse {
  let good_response = crate::common::errors::ApiError::new(200, "Ok".to_string());

  // must check for unique user
  if payload.field == ValidateUserContentType::Email {
    return crate::common::into_response_res(
      user_service
        .email_in_use(&payload.content)
        .await
        .and_then(|in_use| {
          if in_use {
            Ok(crate::common::errors::ApiError::new(
              400,
              "Email in use".to_string(),
            ))
          } else {
            Ok(good_response)
          }
        }),
    );
  }

  good_response.into()
}

// you add the services here.
use actix_web::web::ServiceConfig;
pub fn init_routes(config: &mut ServiceConfig) {
  config.service(signup);
  config.service(login);
  config.service(update_user);
  config.service(logout);
  config.service(validate_user);
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
    let birthday = "1970-01-01".to_string();

    assert_eq!(
      Ok(()),
      SignupPayload {
        email: "me@chucknorris.com".to_string(),
        password: "fafdfdf".to_string(),
        first_name: "first name".to_string(),
        last_name: "last name".to_string(),
        income: 1000.into(),
        location: Location::default(),
        birthday: birthday.clone(),
      }
      .validate()
    );

    // should fail on Negative Income
    assert!(SignupPayload {
      email: "me@chucknorris.com".to_string(),
      password: "fadfdfda".to_string(),
      first_name: "first name".to_string(),
      last_name: "last name".to_string(),
      income: Decimal::new(-1, 0),
      location: Location::default(),
      birthday: birthday.clone(),
    }
    .validate()
    .is_err());

    // fail on bad email
    assert!(SignupPayload {
      email: "bad email".to_string(),
      password: "".to_string(),
      first_name: "first name".to_string(),
      last_name: "last name".to_string(),
      income: 1000.into(),
      location: Location::default(),
      birthday: birthday.clone(),
    }
    .validate()
    .is_err());

    // fail on empty password
    assert!(SignupPayload {
      email: "me@chucknorris.com".to_string(),
      password: "".to_string(),
      first_name: "first name".to_string(),
      last_name: "last name".to_string(),
      income: 1000.into(),
      location: Location::default(),
      birthday: birthday.clone(),
    }
    .validate()
    .is_err());

    // eventually fail on empty name
    assert!(SignupPayload {
      email: "me@chucknorris.com".to_string(),
      password: "fadfdf".to_string(),
      first_name: "".to_string(),
      last_name: "".to_string(),
      income: 1000.into(),
      location: Location::default(),
      birthday: birthday.clone(),
    }
    .validate()
    .is_err());

    // fail on too young
    assert!(SignupPayload {
      email: "me@chucknorris.com".to_string(),
      password: "fadfdf".to_string(),
      first_name: "a".to_string(),
      last_name: "b".to_string(),
      income: 1000.into(),
      location: Location::default(),
      birthday: chrono::Utc::now().format("%Y-%m-%d").to_string(),
    }
    .validate()
    .is_err());
  }
}
