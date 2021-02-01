use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use wither::{mongodb::bson::oid::ObjectId, Model};

#[derive(Model, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Goal {
  #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
  pub id: Option<ObjectId>,
  #[serde(rename = "name")]
  pub name: String,
  #[serde(rename = "start")]
  pub start: i64,
  #[serde(rename = "end")]
  pub end: i64,
  #[serde(rename = "threshold")]
  pub threshold: Decimal,
  #[serde(rename = "metric")]
  pub metric: GoalMetrics,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum GoalMetrics {
  Savings,
  Spending,
  Income,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GoalAndStatus {
  pub goal: Goal,
  pub progress: Decimal,
}
