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
    use crate::services::{timeseries::TimeseriesService, users::UserService};
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

    pub async fn update_plan(
        payload: PlanUpdatePayload,
        mut user: User,
        days: i64,
        user_service: Data<UserService>,
        plaid_client: Data<Arc<Mutex<ApiClient>>>,
    ) -> Result<PlanResponse, ApiError> {
        if user.plans.len() < 1 {
            Err(ApiError::new(500, format!("No plan found in current user")))
        } else {
            let mut plan = user.plans[0].clone();

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
    }

    pub async fn update_plaid_allocation(
        mut user: User,
        days: i64,
        user_service: Data<UserService>,
        plaid_client: Data<Arc<Mutex<ApiClient>>>,
    ) -> Result<PlanResponse, ApiError> {
        let new_alloc = get_plaid_allocation(user.clone(), plaid_client.clone()).await;

        if user.plans.len() < 1 {
            user.plans.push(Plan {
                id: None,
                name: "My Plan".to_string(),
                recurrings: vec![],
                allocations: vec![new_alloc],
                events: vec![],
            });
        } else {
            user.plans[0].allocations.push(new_alloc);
        }

        let plan = user.plans[0].clone();
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
        if user.plans.len() < 1 {
            Err(ApiError::new(500, format!("No plan found in current user")))
        } else {
            let plan = user.plans[0].clone();
            let timeseries =
                TimeseriesService::get_timeseries(user, days, user_service, plaid_client).await?;
            Ok(PlanResponse {
                plan: plan,
                timeseries: timeseries,
            })
        }
    }

    pub async fn delete_plan(
        plan_id: String,
        mut user: User,
        user_service: Data<UserService>,
    ) -> Result<Plan, ApiError> {
        let plan_id_opt = Some(
            ObjectId::with_string(plan_id.as_str())
                .or(Err(ApiError::new(400, "Malformed Object Id".to_string())))?,
        );
        let removed = user
            .plans
            .iter()
            .position(|rec| rec.id == plan_id_opt)
            .ok_or(ApiError::new(
                500,
                format!("No plan with id {} found in current user", plan_id),
            ))
            .and_then(|pos| Ok(user.plans.swap_remove(pos)))?;
        user_service.save(&mut user).await?;

        Ok(removed)
    }

    pub async fn get_plaid_allocation(
        user: User,
        plaid_client: Data<Arc<Mutex<ApiClient>>>,
    ) -> Allocation {
        let mut accounts = Vec::new();
        for item in user.accounts.iter() {
            match crate::services::finchplaid::get_account_data(item, plaid_client.clone()).await {
                Ok(mut res) => accounts.append(&mut res),
                Err(_) => (),
            };
        }

        if user.net_worth > dec!(0.0) {
            generate_plaid_allocation(accounts, user.net_worth.clone())
        } else {
            Allocation {
                description: "Current Holdings".to_string(),
                date: (offset::Utc::now()).timestamp(),
                schema: vec![AllocationChange {
                    asset: Asset {
                        name: "depository".to_string(),
                        class: AssetClass::Cash,
                        annualized_performance: dec!(1.0),
                    },
                    change: dec!(100.0), // for now make everything positive
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
                    .clone()
                    .into_iter()
                    .find(|a| a.class == account_class)
                {
                    Some(act) => act.apy,
                    None => dec!(1.0),
                };

                AllocationChange {
                    asset: Asset {
                        name: a.account_type,
                        class: account_class,
                        annualized_performance: performance,
                    },
                    change: a.balance / net_worth * dec!(100.0),
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

        let test_change = AllocationChange {
            asset: test_asset,
            change: dec!(100.0),
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
            AllocationChange {
                asset: Asset {
                    name: "depository".to_string(),
                    class: AssetClass::Cash,
                    annualized_performance: dec!(1.0),
                },
                change: dec!(50.0),
            },
            AllocationChange {
                asset: Asset {
                    name: "investment".to_string(),
                    class: AssetClass::Equity,
                    annualized_performance: dec!(1.05),
                },
                change: dec!(50.0),
            },
        ];
        let res = PlansService::generate_plaid_allocation(accounts, net_worth);

        for i in 0..2 {
            assert_eq!(target[i], res.schema[i]);
        }
    }
}
