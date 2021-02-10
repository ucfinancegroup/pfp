use serde::{Deserialize, Serialize};
use wither::{mongodb::bson::oid::ObjectId, Model};

#[derive(Model, Clone, Debug, Serialize, Deserialize)]
pub struct FinancialProduct {
  #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
  pub id: Option<ObjectId>,

  pub name: String, // eventually we will want to index on this.
  pub description: String,
  pub perks: Vec<String>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub url: Option<String>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub image_url: Option<String>,

  pub product_info: FinancialProductInfo,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum FinancialProductInfo {
  CreditCard,
  Loan,
  Savings,
  Checking,
  Investment,
  Retirement,
  Other,
}

impl FinancialProduct {
  pub fn new(
    name: &str,
    description: &str,
    perks: Vec<String>,
    url: Option<String>,
    image_url: Option<String>,
    product_info: FinancialProductInfo,
  ) -> Self {
    Self {
      id: None,
      name: name.to_string(),
      description: description.to_string(),
      perks,
      url,
      image_url,
      product_info,
    }
  }
}

#[allow(unused_imports)]
use chrono::TimeZone;
use wither::mongodb::bson::doc;
use wither::prelude::Migrating;

impl Migrating for FinancialProduct {
  fn migrations() -> Vec<Box<dyn wither::Migration>> {
    vec![]
  }
}
