use serde::{Deserialize, Serialize};
use wither::{mongodb::bson::oid::ObjectId, Model};

#[derive(Model, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Insight {
  #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
  pub id: Option<ObjectId>,

  #[serde(rename = "title")]
  pub title: String,

  #[serde(rename = "description")]
  pub description: String,

  #[serde(rename = "insight_type")]
  pub insight_type: InsightTypes,

  #[serde(rename = "dismissed")]
  pub dismissed: bool,

  #[serde(rename = "imageURL", skip_serializing_if = "Option::is_none")]
  pub image: Option<String>,

  #[serde(rename = "generation_time")]
  pub generation_time: i64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum InsightTypes {
  ProductRecommendation,
  Savings,
  Spending,
  Income,
  Goal,
  Incomplete, // for insights that are not yet generated.
}
