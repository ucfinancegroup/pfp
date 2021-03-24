use crate::common::ensure_id;
use crate::models::recurring_model::Recurring;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use validator::Validate;
use validator::ValidationError;
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

impl Default for Allocation {
    fn default() -> Self {
        Allocation {
            id: None,
            description: "default".to_string(),
            date: 0,
            schema: vec![AllocationProportion {
                asset: Asset {
                    name: "Cash".to_string(),
                    class: AssetClass::Cash,
                    annualized_performance: dec!(0.0),
                },
                proportion: dec!(100.0),
            }],
        }
    }
}
#[derive(Validate, Model, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Event {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    #[validate(range(min = 0))]
    pub start: i64,
    #[validate(custom = "transforms_asset_class_unique")]
    pub transforms: Vec<AssetClassChange>,
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
pub struct AssetClassChange {
    pub class: AssetClass,
    pub change: Decimal,
}

#[derive(Validate, Clone, Debug, PartialEq, Serialize, Deserialize, Eq)]
pub struct AllocationProportion {
    pub asset: Asset,
    #[validate(custom = "crate::common::decimal_between_zero_or_hundred")]
    pub proportion: Decimal,
}

fn transforms_asset_class_unique(
    transforms: &Vec<AssetClassChange>,
) -> Result<(), ValidationError> {
    use std::collections::HashMap;

    let mut hm: HashMap<AssetClass, i32> = HashMap::new();
    for t in transforms.iter() {
        let entry = hm.entry(t.class.clone()).or_insert(0);
        if *entry > 0 {
            return Err(ValidationError::new(
                "At most one AssetClassChange allowed per Asset Class",
            ));
        }
        *entry += 1;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_event_validation() {
        let ev = Event {
            id: None,
            name: "Crash".to_string(),
            start: 0,
            transforms: vec![AssetClassChange {
                class: AssetClass::Cash,
                change: dec!(-100),
            }],
        };

        let bad_ev = Event {
            id: None,
            name: "Crash 2".to_string(),
            start: 0,
            transforms: vec![
                AssetClassChange {
                    class: AssetClass::Cash,
                    change: dec!(-100),
                },
                AssetClassChange {
                    class: AssetClass::Cash,
                    change: dec!(-100),
                },
            ],
        };

        assert!(ev.validate().is_ok());
        assert!(bad_ev.validate().is_err());
    }

    #[test]
    fn test_allocation_validation() {
        let alloc = Allocation {
            id: None,
            description: "despacito".to_string(),
            date: 0,
            schema: vec![AllocationProportion {
                asset: Asset {
                    name: "dollars".to_string(),
                    class: AssetClass::Cash,
                    annualized_performance: dec!(0.01),
                },
                proportion: dec!(100.0),
            }],
        };

        let bad_alloc = Allocation {
            id: None,
            description: "despacito".to_string(),
            date: 0,
            schema: vec![AllocationProportion {
                asset: Asset {
                    name: "dollars".to_string(),
                    class: AssetClass::Cash,
                    annualized_performance: dec!(0.01),
                },
                proportion: dec!(50.0),
            }],
        };

        let bad_alloc2 = Allocation {
            id: None,
            description: "despacito".to_string(),
            date: 0,
            schema: vec![
                AllocationProportion {
                    asset: Asset {
                        name: "dollars".to_string(),
                        class: AssetClass::Cash,
                        annualized_performance: dec!(0.01),
                    },
                    proportion: dec!(60.0),
                },
                AllocationProportion {
                    asset: Asset {
                        name: "dollars".to_string(),
                        class: AssetClass::Equity,
                        annualized_performance: dec!(0.01),
                    },
                    proportion: dec!(60.0),
                },
            ],
        };

        println!("{:?}", alloc.validate());

        assert!(alloc.validate().is_ok());
        assert!(bad_alloc.validate().is_err());
        assert!(bad_alloc2.validate().is_err());
    }
}
