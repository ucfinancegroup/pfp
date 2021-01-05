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
  pub principal: i64,
  #[serde(rename = "amount")]
  pub amount: i64,
  #[serde(rename = "interest")]
  pub interest: f32,
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
