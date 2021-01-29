use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Money {
  pub amount: i64,
}

impl From<f64> for Money {
  fn from(f: f64) -> Money {
    Money {
      amount: (f * 100.0).floor() as i64,
    }
  }
}

impl Into<f64> for Money {
  fn into(self) -> f64 {
    (self.amount as f64) / 100.0
  }
}

impl std::ops::Sub for Money {
  type Output = f64;

  fn sub(self, other: Self) -> Self::Output {
    ((self.amount - other.amount) as f64) / 100.0
  }
}
