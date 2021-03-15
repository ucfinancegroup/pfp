use chrono::{DateTime, Datelike, NaiveDateTime, Utc};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};
use wither::{mongodb::bson::oid::ObjectId, Model};

#[derive(Validate, Model, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[validate(schema(function = "validate_recurring", skip_on_field_errors = false))]
pub struct Recurring {
  #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
  pub id: Option<ObjectId>,
  #[serde(rename = "name")]
  pub name: String,
  #[serde(rename = "start")]
  #[validate(range(min = 0))]
  pub start: i64,
  #[serde(rename = "end")]
  #[validate(range(min = 0))]
  pub end: i64,
  #[serde(rename = "principal")]
  pub principal: Decimal,
  #[serde(rename = "amount")]
  pub amount: Decimal,
  #[serde(rename = "interest")]
  #[validate(custom = "crate::common::decimal_at_least_zero")]
  pub interest: Decimal,
  #[serde(rename = "frequency")]
  pub frequency: TimeInterval,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeInterval {
  #[serde(rename = "typ")]
  pub typ: Typ,
  #[serde(rename = "content")]
  pub content: i32,
}

impl TimeInterval {
  pub fn new(typ: Typ, content: i32) -> TimeInterval {
    TimeInterval { typ, content }
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Typ {
  #[serde(rename = "monthly")]
  Monthly,
  #[serde(rename = "annually")]
  Annually,
  #[serde(rename = "daily")]
  Daily,
  #[serde(rename = "weekly")]
  Weekly,
}

impl Recurring {
  // compounds the principal and returns the change (i.e., the wealth increase)
  pub fn compound(&mut self) -> Decimal {
    // divide by 100 to convert interest percentage to pure interest rate
    let change = self.principal * self.interest / dec!(100);

    self.principal += change;

    change
  }

  pub fn is_active(&self, date: &DateTime<Utc>) -> bool {
    let ts = date.timestamp();
    let active_1 = self.start <= ts && self.end > ts;

    if !active_1 {
      return false;
    }

    let naive = NaiveDateTime::from_timestamp(self.start, 0);
    let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);

    use Typ::*;
    let active_today = match self.frequency.typ {
      Daily => true, // always do dailies
      Weekly => datetime.weekday() == date.weekday(),
      Monthly => datetime.day() == date.day(),
      Annually => datetime.ordinal() == date.ordinal(),
    };

    active_today
  }
}

fn validate_recurring(data: &Recurring) -> Result<(), ValidationError> {
  if data.principal == dec!(0) && data.interest == dec!(0) && data.amount != dec!(0) {
    Ok(())
  } else if data.amount == dec!(0) && data.principal != dec!(0) {
    Ok(())
  } else {
    Err(ValidationError::new(
      "Only one of Principal and Amount can be non-zero\
     and Interest must be zero if Amount is non-zero.",
    ))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_recurring_validation() {
    let rec = Recurring {
      id: None,
      name: "ligma".to_string(),
      start: 0,
      end: 10,
      principal: dec!(0.0),
      amount: dec!(-1),
      interest: dec!(0),
      frequency: TimeInterval::new(Typ::Monthly, 1),
    };

    // both amount and principal non-zero
    let bad_rec = Recurring {
      id: None,
      name: "ligma".to_string(),
      start: 0,
      end: 10,
      principal: dec!(1.0),
      amount: dec!(-1),
      interest: dec!(0),
      frequency: TimeInterval::new(Typ::Monthly, 1),
    };

    // amount and interest non-zero
    let bad_rec2 = Recurring {
      id: None,
      name: "ligma".to_string(),
      start: 0,
      end: 10,
      principal: dec!(0),
      amount: dec!(-1),
      interest: dec!(1),
      frequency: TimeInterval::new(Typ::Monthly, 1),
    };

    // bad start/end times
    let bad_rec3 = Recurring {
      id: None,
      name: "ligma".to_string(),
      start: -1,
      end: -1,
      principal: dec!(1),
      amount: dec!(0),
      interest: dec!(1),
      frequency: TimeInterval::new(Typ::Monthly, 1),
    };

    assert!(rec.validate().is_ok());
    assert!(bad_rec.validate().is_err());
    assert!(bad_rec2.validate().is_err());
    assert!(bad_rec3.validate().is_err());
  }
}
