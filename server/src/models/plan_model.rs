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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Allocation {
    pub description: String,
    pub date: i64,
    pub schema: Vec<AllocationChange>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Event {
    pub name: String,
    pub start: i64,
    pub transforms: Vec<Transform>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Transform {
    pub trigger: TimeInterval,
    pub changes: Vec<AssetChange>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Asset {
    pub name: String,
    pub class: String,
    pub annualized_performance: Decimal,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssetChange {
    pub asset: Asset,
    pub change: Decimal,
}

#[derive(Validate, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AllocationChange {
    pub asset: Asset,
    #[validate(custom = "crate::common::decimal_between_zero_or_hundred")]
    pub change: Decimal,
}
