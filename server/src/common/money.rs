use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq)]
pub struct Money {
  pub amount: Decimal,
}

impl Money {
  pub fn new<T: Into<Decimal>>(d: T) -> Self {
    Self { amount: d.into() }
  }
}

impl Default for Money {
  fn default() -> Self {
    Money::new(Decimal::new(0, 0))
  }
}

// impl Into<Decimal> for Money {
//   fn into(self) -> Decimal {
//     self.amount
//   }
// }

impl<T: Into<Decimal>> From<T> for Money {
  fn from(d: T) -> Self {
    Self::new(d)
  }
}

impl<T: Into<Money>> Add<T> for Money {
  type Output = Money;
  fn add(self, other: T) -> Self::Output {
    Money::new(self.amount + other.into().amount)
  }
}

impl<T: Into<Money>> Sub<T> for Money {
  type Output = Money;
  fn sub(self, other: T) -> Self::Output {
    Money::new(self.amount - other.into().amount)
  }
}

impl<T: Into<Money>> Mul<T> for Money {
  type Output = Money;
  fn mul(self, other: T) -> Self::Output {
    Money::new(self.amount * other.into().amount)
  }
}

impl<T: Into<Money>> Div<T> for Money {
  type Output = Money;
  fn div(self, other: T) -> Self::Output {
    Money::new(self.amount / other.into().amount)
  }
}
