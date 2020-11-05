use serde::{Serialize, Deserialize};
use rand::Rng;

use argon2::{self, Config};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
  password: String,
}

impl User {
  #[allow(dead_code)]
  pub fn new() -> User { User {password:"hi".to_string()} }

  pub fn new_from_signup(data: SignupPayload) -> User {
    User {password:"Hi".to_string()}
  }

  fn hash_password(plaintext: String) -> Result<String, argon2::Error> {
    let salt: [u8; 32] = rand::thread_rng().gen::<[u8; 32]>();
    let config = Config::default();
    argon2::hash_encoded(plaintext.as_bytes(), &salt, &config)
  }

  fn compare_password(&self, plaintext: String) -> Result<bool, argon2::Error> {
    argon2::verify_encoded(&self.password.as_str(), plaintext.as_bytes())
  }
}

#[derive(Deserialize)]
pub struct SignupPayload {
  pub email: String,
  pub password: String,
  pub first_name: String,
  pub last_name: String,
}

pub trait Validation {
  fn validate(&self) -> Result<(), String>;
}

impl Validation for SignupPayload {
  fn validate(&self) -> Result<(), String> {
    return Ok(())
  }
}
