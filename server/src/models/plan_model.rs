use chrono::DateTime;
use models::recurring_model::{Recurring, TimeInterval};
use serde::{Deserialize, Serialize};
use serde_json::Map;
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
    pub date: DateTime,
    pub schema: Vec<AssetChange>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Event {
    pub name: String,
    pub start: DateTime,
    pub transforms: Vec<Transfrom>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Transfrom {
    pub trigger: TimeInterval,
    pub changes: Vec<AssetChange>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Asset {
    pub name: String,
    pub class: String,
    pub annualized_performance: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssetChange {
    pub asset: Asset,
    pub change: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AllocationChange {
    pub asset: Asset,
    #[validate(range(min = 0, max = 100))]
    pub change: f64,
}
