#[allow(non_snake_case)]
pub mod PlansService {
    use crate::common::errors::ApiError;
    use crate::controllers::plans_controller::PlanNewPayload;
    use crate::controllers::timeseries_controller::TimeseriesResponse;
    use crate::models::plan_model::*;
    use crate::models::recurring_model::*;
    use crate::models::user_model::User;
    use crate::services::finchplaid::ApiClient;
    use crate::services::{timeseries::TimeseriesService, users::UserService};
    use actix_web::web::Data;
    use chrono::offset;
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

        user.plans.push(plan.clone());

        user_service.save(&mut user).await?;

        let timeseries =
            TimeseriesService::get_timeseries(user, days, user_service, plaid_client).await?;

        Ok(PlanResponse {
            plan: plan,
            timeseries: timeseries,
        })
    }

    pub async fn get_plan(
        plan_id: String,
        user: User,
        days: i64,
        user_service: Data<UserService>,
        plaid_client: Data<Arc<Mutex<ApiClient>>>,
    ) -> Result<PlanResponse, ApiError> {
        let plan_id_opt = Some(
            ObjectId::with_string(plan_id.as_str())
                .or(Err(ApiError::new(400, "Malformed Object Id".to_string())))?,
        );

        let plan = match user
            .plans
            .clone()
            .into_iter()
            .find(|rec| rec.id == plan_id_opt)
        {
            Some(p) => p,
            None => {
                return Err(ApiError::new(
                    400,
                    format!("No plan with id {} found in current user", plan_id),
                ))
            }
        };

        let timeseries =
            TimeseriesService::get_timeseries(user, days, user_service, plaid_client).await?;

        Ok(PlanResponse {
            plan: plan,
            timeseries: timeseries,
        })
    }

    pub async fn update_plan(
        payload: PlanNewPayload,
        plan_id: String,
        mut user: User,
        days: i64,
        user_service: Data<UserService>,
        plaid_client: Data<Arc<Mutex<ApiClient>>>,
    ) -> Result<PlanResponse, ApiError> {
        let plan_id_opt = Some(
            ObjectId::with_string(plan_id.as_str())
                .or(Err(ApiError::new(400, "Malformed Object Id".to_string())))?,
        );

        let mut plan: Plan = payload.into();
        plan.set_id(plan_id_opt.unwrap().clone());
        let updated = user
            .plans
            .iter_mut()
            .find(|rec| rec.id == plan.id)
            .ok_or(ApiError::new(
                400,
                format!("No plan with id {} found in current user", plan_id),
            ))
            .and_then(|rec| {
                *rec = plan.clone();
                Ok(plan)
            })?;
        user_service.save(&mut user).await?;

        let timeseries =
            TimeseriesService::get_timeseries(user, days, user_service, plaid_client).await?;

        Ok(PlanResponse {
            plan: updated,
            timeseries: timeseries,
        })
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
                400,
                format!("No plan with id {} found in current user", plan_id),
            ))
            .and_then(|pos| Ok(user.plans.swap_remove(pos)))?;
        user_service.save(&mut user).await?;

        Ok(removed)
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
