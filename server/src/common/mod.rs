pub mod errors;

pub trait Validation {
  fn validate(&self) -> Result<(), String>;
}
