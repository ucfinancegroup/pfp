#[allow(non_snake_case)]
pub mod PlansService {
    use crate::common::errors::ApiError;
    use crate::controllers::plaid_controller::AccountSuccess;
    use crate::controllers::plans_controller::{PlanNewPayload, PlanUpdatePayload};
    use crate::controllers::timeseries_controller::TimeseriesResponse;
    use crate::models::plan_model::*;
    use crate::models::recurring_model::*;
    use crate::models::user_model::User;
    use crate::services::finchplaid::ApiClient;
    use crate::services::{
        snapshots::SnapshotService, timeseries::TimeseriesService, users::UserService,
    };
    use actix_web::web::Data;
    use chrono::offset;
    use rust_decimal::Decimal;
    use rust_decimal_macros::dec;
    use serde::{Deserialize, Serialize};
    use std::sync::{Arc, Mutex};
    use wither::{mongodb::bson::oid::ObjectId, Model};

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
    pub struct PlanResponse {
        pub plan: Plan,
        pub timeseries: TimeseriesResponse,
    }

    pub fn get_user_plan(user: &User) -> Plan {
        if user.plans.len() < 1 {
            generate_sample_plan()
        } else {
            user.plans[0].clone()
        }
    }

    pub async fn new_plan(
        payload: PlanNewPayload,
        mut user: User,
        days: i64,
        user_service: Data<UserService>,
        plaid_client: Data<Arc<Mutex<ApiClient>>>,
    ) -> Result<PlanResponse, ApiError> {
        let mut plan: Plan = payload.into();
        plan.set_id(ObjectId::new());

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
        plaid_client: Data<Arc<Mutex<ApiClient>>>,
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
        plaid_client: Data<Arc<Mutex<ApiClient>>>,
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
        plaid_client: Data<Arc<Mutex<ApiClient>>>,
    ) -> Result<PlanResponse, ApiError> {
        let last_snapshot = SnapshotService::get_last_snapshot(
            &(user_service
                .get_snapshots(&mut user, plaid_client.clone())
                .await?),
        );
        let net_worth = last_snapshot.net_worth.amount;

        let new_alloc = get_plaid_allocation(user.clone(), plaid_client.clone(), net_worth).await;
        let plan = get_user_plan(&user);

        user_service.add_plaid_plan(user.clone(), new_alloc).await?;

        let timeseries =
            TimeseriesService::get_timeseries(user, days, user_service, plaid_client).await?;

        Ok(PlanResponse {
            plan: plan,
            timeseries: timeseries,
        })
    }

    pub async fn get_plaid_allocation(
        user: User,
        plaid_client: Data<Arc<Mutex<ApiClient>>>,
        net_worth: Decimal,
    ) -> Allocation {
        let mut accounts = Vec::new();
        for item in user.accounts.iter() {
            match crate::services::finchplaid::get_account_data(item, plaid_client.clone()).await {
                Ok(mut res) => accounts.append(&mut res),
                Err(_) => (),
            };
        }

        if net_worth > dec!(0.0) {
            generate_plaid_allocation(accounts, net_worth)
        } else {
            Allocation {
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
        }
    }

    pub fn generate_plaid_allocation(
        accounts: Vec<AccountSuccess>,
        net_worth: Decimal,
    ) -> Allocation {
        let default_percentages = get_asset_classes_and_default_apys();
        let asset_percentages = accounts
            .into_iter()
            .map(|a| {
                let account_class = match a.account_type.as_str() {
                    "depository" => AssetClass::Cash,
                    "credit" => AssetClass::Loan,
                    "loan" => AssetClass::Loan,
                    "investment" => AssetClass::Equity, // for now classify all investments as broad equities
                    _ => AssetClass::Cash,
                };

                let performance = match default_percentages
                    .iter()
                    .find(|a| a.class == account_class)
                {
                    Some(act) => act.apy.clone(),
                    None => dec!(1.0),
                };

                AllocationProportion {
                    asset: Asset {
                        name: a.account_type,
                        class: account_class,
                        annualized_performance: performance,
                    },
                    proportion: a.balance / net_worth * dec!(100.0),
                }
            })
            .collect();

        Allocation {
            description: "Current Holdings".to_string(),
            date: (offset::Utc::now()).timestamp(),
            schema: asset_percentages,
        }
    }

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
            description: String::from("A Test Allocation"),
            date: offset::Utc::now().timestamp(),
            schema: vec![test_change],
        };
        let allocations = vec![test_allocation];

        let events = vec![Event {
            name: String::from("Test Event"),
            start: offset::Utc::now().timestamp(),
            transforms: vec![Transform {
                trigger: TimeInterval {
                    typ: Typ::Monthly,
                    content: 1,
                },
                changes: vec![AssetChange {
                    asset: Asset {
                        name: String::from("AAPL"),
                        class: AssetClass::Equity,
                        annualized_performance: dec!(1.2),
                    },
                    change: dec!(10.0),
                }],
            }],
        }];

        Plan {
            id: None,
            name: String::from("Test Plan"),
            recurrings: recurrings,
            allocations: allocations,
            events: events,
        }
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
                class: Loan,
                apy: dec!(0.97),
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
                    name: "depository".to_string(),
                    class: AssetClass::Cash,
                    annualized_performance: dec!(1.0),
                },
                proportion: dec!(50.0),
            },
            AllocationProportion {
                asset: Asset {
                    name: "investment".to_string(),
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
