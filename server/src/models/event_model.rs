use chrono::DateTime;
use models::recurring_model::TimeInterval;
use serde::{Deserialize, Serialize};
use serde_json::Map;
use wither::{mongodb::bson::oid::ObjectId, Model};

pub struct Event {
    pub name: String,
    pub start: DateTime,
    pub transforms: Vec<Transfrom>,
}

pub struct Transfrom {
    pub trigger: TimeInterval,
    pub change: Map<String, f64>,
}

pub struct Asset {
    pub name: String,
    pub class: String,
    pub annualized_performance: String,
}
