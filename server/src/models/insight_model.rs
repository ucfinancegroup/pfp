use chrono::Utc;
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

impl Insight {
  pub fn new(
    title: String,
    description: String,
    insight_type: InsightTypes,
    image: Option<String>,
  ) -> Insight {
    Insight {
      id: Some(ObjectId::new()),
      title,
      description,
      insight_type,
      dismissed: false,
      image,
      generation_time: Utc::now().timestamp(),
    }
  }
}

impl Default for Insight {
  fn default() -> Insight {
    Insight {
      id: None,
      title: "".to_string(),
      description: "".to_string(),
      insight_type: InsightTypes::Incomplete,
      dismissed: false,
      image: None,
      generation_time: Utc::now().timestamp(),
    }
  }
}

impl crate::common::Examples for Insight {
  type Output = Self;
  fn examples() -> Vec<Self::Output> {
    vec![
      Insight::new(
        "Wealthfront Cash Account".to_string(),
        "Consider a Wealthfront Cash Account to boost your savings APY (0.35%).".to_string(),
        InsightTypes::ProductRecommendation,
        Some("https://theme.zdassets.com/theme_assets/586236/49e4904c4910a8ebf63b67d41f9b98b6b0933275.png".to_string()),
      ),
      Insight::new(
        "Savings Insight".to_string(),
        "You are saving more per month than 63% of users in your income bracket!".to_string(),
        InsightTypes::Savings,
        None,
      ),
    ]
  }
}
