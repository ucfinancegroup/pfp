use rand::Rng;
use serde::{Deserialize, Serialize};

use argon2::{self, Config};

use mongodb::{
  bson::{bson, doc, oid::ObjectId},
  sync::Collection,
};

use actix_web::HttpResponse;

use crate::common::{errors::ApiError, Validation};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct User {
  pub _id: ObjectId,
  pub email: String,
  password: String,
  pub first_name: String,
  pub last_name: String,
  pub income: f64,
  // recurrings: Vec<ObjectId>,
  // snapshots: Vec<ObjectId>,
  // accounts: Vec<ObjectId>,
}

impl User {
  pub fn new_from_signup(data: SignupPayload, col: Collection) -> Result<User, ApiError> {
    if let Err(e) = data.validate() {
      return Err(ApiError::new(400, e));
    }

    // check for unused email
    if let Ok(Some(_)) = col.find_one(Some(doc! {"email": data.email.clone()}), None) {
      return Err(ApiError::new(400, "Email is in use".to_string()));
    }

    User::hash_password(data.password.clone())
      .map_err(|_| ApiError::new(400, "Password hashing failed".to_string()))
      .and_then(|password_hash| {
        let user = User {
          _id: ObjectId::new(),
          email: data.email,
          password: password_hash,
          first_name: data.first_name,
          last_name: data.last_name,
          income: data.income,
        };

        col
          .insert_one(bson!(user.clone()).as_document().unwrap().clone(), None)
          .and_then(|_| Ok(user))
          .map_err(|_| ApiError::new(500, "Database Error".to_string()))
      })
  }

  pub fn new_from_login(data: LoginPayload, col: Collection) -> Result<User, ApiError> {
    if let Err(e) = data.validate() {
      return Err(ApiError::new(400, e));
    }

    // search db for user
    let search_db_res = col
      .find_one(Some(doc! {"email": data.email.clone()}), None)
      .map_err(|_| ApiError::new(500, "DB Error".to_string()));

    // check if user found and parse to User
    let got_user_res: Result<User, ApiError> = search_db_res.and_then(|user_opt| {
      user_opt
        .ok_or(ApiError::new(500, "User not found".to_string()))
        .and_then(|user| {
          bson::from_bson(user.into())
            .map_err(|_| ApiError::new(500, "user format error".to_string()))
        })
    });

    // verify password, return user if good
    got_user_res.and_then(|user| {
      user
        .compare_password(data.password)
        .and_then(|is_correct_password| {
          if is_correct_password {
            Ok(user)
          } else {
            Err(ApiError::new(401, "Incorrect user or password".to_string()))
          }
        })
    })
  }

  fn hash_password(plaintext: String) -> Result<String, ApiError> {
    let salt: [u8; 32] = rand::thread_rng().gen::<[u8; 32]>();
    let config = Config::default();
    argon2::hash_encoded(plaintext.as_bytes(), &salt, &config).map_err(|_| {
      ApiError::new(
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
        "Password hashing failed".to_string(),
      )
    })
  }

  fn compare_password(&self, plaintext: String) -> Result<bool, ApiError> {
    argon2::verify_encoded(&self.password.as_str(), plaintext.as_bytes()).map_err(|_| {
      ApiError::new(
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
        "Password checking failed".to_string(),
      )
    })
  }
}

impl std::convert::From<User> for mongodb::bson::Bson {
  fn from(s: User) -> mongodb::bson::Bson {
    mongodb::bson::to_bson(&s).unwrap()
  }
}

#[derive(Deserialize, PartialEq)]
pub struct SignupPayload {
  pub email: String,
  pub password: String,
  pub first_name: String,
  pub last_name: String,
  pub income: f64,
}

impl Validation for SignupPayload {
  fn validate(&self) -> Result<(), String> {
    return Ok(());
  }
}

#[derive(Serialize, PartialEq)]
pub struct SignupResponse {
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

impl Into<HttpResponse> for SignupResponse {
  fn into(self) -> HttpResponse {
    HttpResponse::Ok().json(self)
  }
}

#[derive(Deserialize, PartialEq)]
pub struct LoginPayload {
  pub email: String,
  pub password: String,
}

impl Validation for LoginPayload {
  fn validate(&self) -> Result<(), String> {
    return Ok(());
  }
}

pub type LoginResponse = SignupResponse;

#[cfg(test)]
mod test {
  use super::*;

  #[allow(non_snake_case)]
  fn test_LoginResponse() {}

  #[allow(non_snake_case)]
  fn test_SignupResponse() {}

  #[allow(non_snake_case)]
  fn test_LoginPayload() {
    assert_eq!(
      Ok(()),
      LoginPayload {
        email: "me@chucknorris.com".to_string(),
        password: "password".to_string(),
      }
      .validate()
    );

    // eventually should be not ok
    assert_eq!(
      Ok(()),
      LoginPayload {
        email: "not an email".to_string(),
        password: "password".to_string(),
      }
      .validate()
    );

    // eventually should be not ok
    assert_eq!(
      Ok(()),
      LoginPayload {
        email: "me@chucknorris.com".to_string(),
        password: "".to_string(),
      }
      .validate()
    );
  }

  #[allow(non_snake_case)]
  fn test_SignupPayload() {
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

    // should eventually fail on Negative Income
    assert_eq!(
      Ok(()),
      SignupPayload {
        email: "me@chucknorris.com".to_string(),
        password: "fadfdfda".to_string(),
        first_name: "first name".to_string(),
        last_name: "last name".to_string(),
        income: -1 as f64
      }
      .validate()
    );

    // fail on bad email
    assert_eq!(
      Ok(()),
      SignupPayload {
        email: "bad email".to_string(),
        password: "".to_string(),
        first_name: "first name".to_string(),
        last_name: "last name".to_string(),
        income: 1000 as f64
      }
      .validate()
    );

    // eventually fail on empty password
    assert_eq!(
      Ok(()),
      SignupPayload {
        email: "me@chucknorris.com".to_string(),
        password: "".to_string(),
        first_name: "first name".to_string(),
        last_name: "last name".to_string(),
        income: 1000 as f64
      }
      .validate()
    );

    // eventually fail on empty name
    assert_eq!(
      Ok(()),
      SignupPayload {
        email: "me@chucknorris.com".to_string(),
        password: "fadfdf".to_string(),
        first_name: "".to_string(),
        last_name: "".to_string(),
        income: 1000 as f64
      }
      .validate()
    );
  }

  #[allow(non_snake_case)]
  fn test_PasswordHashing() {
    let hashed = User::hash_password("password".to_string()).unwrap();

    let user = User {
      _id: ObjectId::new(),
      email: "email".to_string(),
      password: hashed,
      first_name: "first_name".to_string(),
      last_name: "last_name".to_string(),
      income: 0.0,
    };

    assert_eq!(Ok(true), user.compare_password("password".to_string()));
    assert_eq!(Ok(false), user.compare_password("bad password".to_string()));
  }
}
