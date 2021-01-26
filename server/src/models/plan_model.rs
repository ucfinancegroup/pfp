use chrono::DateTime;
use models::recurring_model::TimeInterval;
use serde::{Deserialize, Serialize};
use serde_json::Map;
use wither::{mongodb::bson::oid::ObjectId, Model};

#[derive(Model, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Plan {}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Allocation {}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Event {
    pub name: String,
    pub start: DateTime,
    pub transforms: Vec<Transfrom>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Transfrom {
    pub trigger: TimeInterval,
    pub change: Map<String, f64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Asset {
    pub name: String,
    pub class: String,
    pub annualized_performance: String,
}
