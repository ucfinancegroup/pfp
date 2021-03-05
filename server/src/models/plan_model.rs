use crate::common::ensure_id;
use crate::models::recurring_model::{Recurring, TimeInterval};
use actix_web_validator::Validate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use wither::{mongodb::bson::oid::ObjectId, Model};

#[derive(Model, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Plan {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub recurrings: Vec<Recurring>,
    pub allocations: Vec<Allocation>,
    pub events: Vec<Event>,
}

impl Plan {
    pub fn ensure_ids(mut self) -> Self {
        ensure_id(&mut self);
        self.recurrings.iter_mut().for_each(ensure_id);
        self.allocations.iter_mut().for_each(ensure_id);
        self.events.iter_mut().for_each(ensure_id);
        self
    }
}

#[derive(Model, Validate, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Allocation {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub description: String,
    pub date: i64,
    #[validate(custom = "crate::common::allocation_schema_sum_around_100")]
    pub schema: Vec<AllocationProportion>,
}

#[derive(Model, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Event {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub start: i64,
    pub transforms: Vec<Transform>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Transform {
    pub trigger: TimeInterval,
    pub changes: Vec<AssetChange>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Eq)]
pub struct Asset {
    pub name: String,
    pub class: AssetClass,
    pub annualized_performance: Decimal,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Eq, Hash)]
#[serde(tag = "typ", content = "content")]
pub enum AssetClass {
    Cash,
    Equity,
    Etf,
    Fixed,
    MutualFund,
    Other,
    Custom(String),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssetClassAndApy {
    pub class: AssetClass,
    pub apy: Decimal,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssetChange {
    pub asset: Asset,
    pub change: Decimal,
}

#[derive(Validate, Clone, Debug, PartialEq, Serialize, Deserialize, Eq)]
pub struct AllocationProportion {
    pub asset: Asset,
    #[validate(custom = "crate::common::decimal_between_zero_or_hundred")]
    pub proportion: Decimal,
}
