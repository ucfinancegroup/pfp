use chrono::{DateTime, Datelike, NaiveDateTime, Utc};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use wither::{mongodb::bson::oid::ObjectId, Model};

#[derive(Model, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Recurring {
  #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
  pub id: Option<ObjectId>,
  #[serde(rename = "name")]
  pub name: String,
  #[serde(rename = "start")]
  pub start: i64,
  #[serde(rename = "end")]
  pub end: i64,
  #[serde(rename = "principal")]
  pub principal: Decimal,
  #[serde(rename = "amount")]
  pub amount: Decimal,
  #[serde(rename = "interest")]
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
