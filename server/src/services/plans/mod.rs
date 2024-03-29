#[allow(non_snake_case)]
pub mod PlansService {
    use crate::common::errors::ApiError;
    use crate::controllers::plaid_controller::AccountSuccess;
    use crate::controllers::plans_controller::{PlanNewPayload, PlanUpdatePayload};
    use crate::controllers::timeseries_controller::TimeseriesResponse;
    use crate::models::plan_model::*;
    use crate::models::recurring_model::*;
    use crate::models::user_model::User;
    use crate::services::{
        finchplaid::ApiClient, snapshots::SnapshotService, timeseries::TimeseriesService,
        users::UserService,
    };
    use actix_web::web::Data;
    use bson::oid::ObjectId;
    use chrono::offset;
    use rust_decimal::Decimal;
    use rust_decimal_macros::dec;
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;
    use validator::Validate;

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
    pub struct PlanResponse {
        pub plan: Plan,
        pub timeseries: TimeseriesResponse,
    }

    pub fn get_user_plan(user: &User) -> Plan {
        if user.plans.len() < 1 {
            let alloc = Allocation {
                id: None,
                description: "Just cash!".to_string(),
                date: chrono::Utc::now().timestamp(),
                schema: vec![AllocationProportion {
                    asset: Asset {
                        name: "Dollars".to_string(),
                        class: AssetClass::Cash,
                        annualized_performance: dec!(1.01),
                    },
                    proportion: dec!(100),
                }],
            };

            assert!(alloc.validate().is_ok());

            Plan {
                id: None,
                name: "Your default Plan".to_string(),
                recurrings: vec![],
                events: vec![],
                allocations: vec![alloc],
            }
        } else {
            user.plans[0].clone()
        }
        .ensure_ids()
    }

    pub async fn new_plan(
        payload: PlanNewPayload,
        mut user: User,
        days: i64,
        user_service: Data<UserService>,
        plaid_client: Data<ApiClient>,
    ) -> Result<PlanResponse, ApiError> {
        let plan: Plan = payload.into();

        if user.plans.len() < 1 {
            user.plans.push(plan.clone());
        } else {
            user.plans[0] = plan.clone();
        }

        user_service.save(&mut user).await?;

        let timeseries =
            TimeseriesService::get_timeseries(user, days, user_service, plaid_client).await?;

        Ok(PlanResponse {
            plan: plan,
            timeseries: timeseries,
        })
    }

    pub async fn get_plan(
        user: User,
        days: i64,
        user_service: Data<UserService>,
        plaid_client: Data<ApiClient>,
    ) -> Result<PlanResponse, ApiError> {
        let plan = get_user_plan(&user);

        let timeseries =
            TimeseriesService::get_timeseries(user, days, user_service, plaid_client).await?;

        Ok(PlanResponse {
            plan: plan,
            timeseries: timeseries,
        })
    }

    pub async fn delete_plan(
        mut user: User,
        user_service: Data<UserService>,
    ) -> Result<Plan, ApiError> {
        let removed = get_user_plan(&user);
        user.plans = vec![];
        user_service.save(&mut user).await?;

        Ok(removed)
    }

    pub async fn update_plan(
        payload: PlanUpdatePayload,
        mut user: User,
        days: i64,
        user_service: Data<UserService>,
        plaid_client: Data<ApiClient>,
    ) -> Result<PlanResponse, ApiError> {
        let mut plan = get_user_plan(&user);

        if let Some(name) = payload.name {
            plan.name = name;
        }

        if let Some(recurrings) = payload.recurrings {
            plan.recurrings = recurrings;
        }

        if let Some(allocations) = payload.allocations {
            plan.allocations = allocations;
        }

        if let Some(events) = payload.events {
            plan.events = events;
        }

        plan = plan.ensure_ids();

        if user.plans.len() < 1 {
            user.plans.push(plan.clone());
        } else {
            user.plans[0] = plan.clone();
        }

        user_service.save(&mut user).await?;

        let timeseries =
            TimeseriesService::get_timeseries(user, days, user_service, plaid_client).await?;

        Ok(PlanResponse {
            plan: plan,
            timeseries: timeseries,
        })
    }

    pub async fn update_plaid_allocation(
        mut user: User,
        days: i64,
        user_service: Data<UserService>,
        plaid_client: Data<ApiClient>,
    ) -> Result<PlanResponse, ApiError> {
        let last_snapshot = SnapshotService::get_last_snapshot(
            &(user_service
                .get_snapshots(&mut user, plaid_client.clone())
                .await?),
        );
        let net_worth = last_snapshot.net_worth.amount;

        let plan = user_service
            .add_plaid_plan(
                user.clone(),
                user_service.clone(),
                plaid_client.clone(),
                net_worth,
            )
            .await?;

        let timeseries =
            TimeseriesService::get_timeseries(user, days, user_service, plaid_client).await?;

        Ok(PlanResponse {
            plan: plan,
            timeseries: timeseries,
        })
    }

    pub async fn get_plaid_allocation(
        user: &User,
        user_service: Data<UserService>,
        plaid_client: Data<ApiClient>,
        net_worth: Decimal,
    ) -> Result<Allocation, ApiError> {
        let accounts = user_service
            .get_accounts(user, plaid_client, false)
            .await?
            .accounts;

        let res = if net_worth > dec!(0.0) {
            generate_plaid_allocation(accounts, net_worth)
        } else {
            Allocation {
                id: None,
                description: "Current Holdings".to_string(),
                date: (offset::Utc::now()).timestamp(),
                schema: vec![AllocationProportion {
                    asset: Asset {
                        name: "depository".to_string(),
                        class: AssetClass::Cash,
                        annualized_performance: dec!(1.0),
                    },
                    proportion: dec!(100.0), // for now make everything positive
                }],
            }
        };

        Ok(res)
    }

    pub fn generate_plaid_allocation(
        accounts: Vec<AccountSuccess>,
        net_worth: Decimal,
    ) -> Allocation {
        let default_percentages = get_asset_classes_and_default_apys()
            .into_iter()
            .map(|class_and_apy| (class_and_apy.class, class_and_apy.apy))
            .collect::<HashMap<_, _>>();

        let asset_percentages = accounts
            .into_iter()
            .filter_map(|account| {
                let t = account.account_type.as_str();

                let asset_class = match t {
                    "depository" => Some(AssetClass::Cash),
                    "investment" => Some(AssetClass::Equity), // for now classify all investments as broad equities
                    _ => None,
                };

                asset_class.map(|c| (c, account.balance, account.name))
            })
            .map(|(asset_class, balance, account_name)| {
                let performance = default_percentages
                    .get(&asset_class)
                    .cloned()
                    .or(Some(dec!(1.0)))
                    .unwrap();

                AllocationProportion {
                    asset: Asset {
                        name: account_name,
                        class: asset_class,
                        annualized_performance: performance,
                    },
                    proportion: balance / net_worth * dec!(100.0),
                }
            })
            .collect();

        Allocation {
            id: None,
            description: "Current Holdings".to_string(),
            date: (offset::Utc::now()).timestamp(),
            schema: asset_percentages,
        }
    }

    // TODO (not this)
    pub fn generate_sample_plan() -> Plan {
        let recurrings = vec![Recurring {
            id: None,
            name: String::from("Test Recurring"),
            start: (offset::Utc::now()).timestamp(),
            end: (offset::Utc::now()).timestamp(),
            principal: dec!(0.0),
            amount: dec!(0.0),
            interest: dec!(0.0),
            frequency: TimeInterval {
                typ: Typ::Monthly,
                content: 1,
            },
        }];

        let test_asset = Asset {
            name: String::from("Finch Savings Account"),
            class: AssetClass::Cash,
            annualized_performance: dec!(1.05),
        };

        let test_change = AllocationProportion {
            asset: test_asset,
            proportion: dec!(100.0),
        };

        let test_allocation = Allocation {
            id: None,
            description: String::from("A Test Allocation"),
            date: offset::Utc::now().timestamp(),
            schema: vec![test_change],
        };
        let allocations = vec![test_allocation];

        let events = vec![Event {
            id: None,
            name: String::from("Test Event"),
            start: offset::Utc::now().timestamp(),
            transforms: vec![AssetClassChange {
                class: AssetClass::Equity,
                change: dec!(10.0),
            }],
        }];

        Plan {
            id: None,
            name: String::from("Test Plan"),
            recurrings: recurrings,
            allocations: allocations,
            events: events,
        }
        .ensure_ids()
    }

    pub fn generate_sample_events() -> Vec<Event> {
        vec![
            Event {
                id: Some(ObjectId::new()),
                name: "COVID-19".to_string(),
                start: offset::Utc::now().timestamp(),
                transforms: vec![
                    AssetClassChange {
                        class: AssetClass::Equity,
                        change: dec!(-30.0),
                    },
                    AssetClassChange {
                        class: AssetClass::Etf,
                        change: dec!(-25.0),
                    },
                    AssetClassChange {
                        class: AssetClass::MutualFund,
                        change: dec!(-22.0),
                    },
                ],
            },
            Event {
                id: Some(ObjectId::new()),
                name: "Meme Stocks Pop Off".to_string(),
                start: offset::Utc::now().timestamp(),
                transforms: vec![AssetClassChange {
                    class: AssetClass::Equity,
                    change: dec!(400.0),
                }],
            },
            Event {
                id: Some(ObjectId::new()),
                name: "Bull Market".to_string(),
                start: offset::Utc::now().timestamp(),
                transforms: vec![
                    AssetClassChange {
                        class: AssetClass::Equity,
                        change: dec!(200.0),
                    },
                    AssetClassChange {
                        class: AssetClass::Etf,
                        change: dec!(140.0),
                    },
                    AssetClassChange {
                        class: AssetClass::MutualFund,
                        change: dec!(160.0),
                    },
                    AssetClassChange {
                        class: AssetClass::Fixed,
                        change: dec!(-0.5),
                    },
                    AssetClassChange {
                        class: AssetClass::Cash,
                        change: dec!(-2.0),
                    },
                ],
            },
            Event {
                id: Some(ObjectId::new()),
                name: "Zimbabwe Inflation".to_string(),
                start: offset::Utc::now().timestamp(),
                transforms: vec![AssetClassChange {
                    class: AssetClass::Cash,
                    change: dec!(-99.99),
                }],
            },
        ]
    }

    pub fn get_asset_classes_and_default_apys() -> Vec<AssetClassAndApy> {
        use AssetClass::*;
        vec![
            AssetClassAndApy {
                class: Cash,
                apy: dec!(1.00),
            },
            AssetClassAndApy {
                class: Equity,
                apy: dec!(1.05),
            },
            AssetClassAndApy {
                class: Etf,
                apy: dec!(1.10),
            },
            AssetClassAndApy {
                class: Fixed,
                apy: dec!(1.02),
            },
            AssetClassAndApy {
                class: MutualFund,
                apy: dec!(1.20),
            },
        ]
    }
}

#[cfg(test)]
mod test {
    use crate::controllers::plaid_controller::AccountSuccess;
    use crate::models::plan_model::*;
    use crate::services::plans::PlansService;
    use rust_decimal_macros::dec;

    fn generate_test_accounts() -> Vec<AccountSuccess> {
        vec![
            AccountSuccess {
                item_id: "blah".to_string(),
                name: "blah".to_string(),
                balance: dec!(500),
                account_type: "depository".to_string(),
                account_id: "blah".to_string(),
            },
            AccountSuccess {
                item_id: "blah2".to_string(),
                name: "blah2".to_string(),
                balance: dec!(500),
                account_type: "investment".to_string(),
                account_id: "blah".to_string(),
            },
        ]
    }

    #[test]
    fn test_plaid_allocation_generation() {
        let net_worth = dec!(1000.0);
        let accounts = generate_test_accounts();

        let target = vec![
            AllocationProportion {
                asset: Asset {
                    name: "blah".to_string(),
                    class: AssetClass::Cash,
                    annualized_performance: dec!(1.0),
                },
                proportion: dec!(50.0),
            },
            AllocationProportion {
                asset: Asset {
                    name: "blah2".to_string(),
                    class: AssetClass::Equity,
                    annualized_performance: dec!(1.05),
                },
                proportion: dec!(50.0),
            },
        ];
        let res = PlansService::generate_plaid_allocation(accounts, net_worth);

        assert_eq!(target, res.schema);
    }
}
