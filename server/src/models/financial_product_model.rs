use rust_decimal::Decimal;
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
  CreditCard { apr: Decimal },
  Loan { apr: Decimal },
  Savings { apy: Decimal },
  Checking { apy: Decimal },
  Investment,
  Retirement,
  Other,
}
