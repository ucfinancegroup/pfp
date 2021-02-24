use crate::models::plan_model::*;
use crate::models::recurring_model::Recurring;
use crate::models::user_model::User;
use crate::services::finchplaid::ApiClient;
use crate::services::{plans::PlansService, users::UserService};
use actix_web::{
    delete, get, post, put,
    web::{Data, Path, ServiceConfig},
    HttpResponse,
};
use actix_web_validator::{Json, Validate};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Validate, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlanNewPayload {
    pub name: String,
    pub recurrings: Vec<Recurring>,
    pub allocations: Vec<Allocation>,
    pub events: Vec<Event>,
}

impl Into<Plan> for PlanNewPayload {
    fn into(self) -> Plan {
        Plan {
            id: None,
            name: self.name,
            recurrings: self.recurrings,
            allocations: self.allocations,
            events: self.events,
        }
    }
}

#[get("/plan/example")]
pub async fn get_example(_: User) -> HttpResponse {
    crate::common::into_response(PlansService::generate_sample_plan())
}

#[get("/plans")]
pub async fn get_plans(user: User) -> HttpResponse {
    crate::common::into_response(user.plans)
}

#[get("/plan/{id}")]
pub async fn get_plan(
    user: User,
    Path(plan_id): Path<String>,
    user_service: Data<UserService>,
    plaid_client: Data<Arc<Mutex<ApiClient>>>,
) -> HttpResponse {
    crate::common::into_response_res(
        PlansService::get_plan(plan_id, user, 365, user_service, plaid_client).await,
    )
}

#[get("/plan/{id}/{days}")]
pub async fn get_plan_with_days(
    user: User,
    Path((plan_id, plan_days)): Path<(String, i64)>,
    user_service: Data<UserService>,
    plaid_client: Data<Arc<Mutex<ApiClient>>>,
) -> HttpResponse {
    crate::common::into_response_res(
        PlansService::get_plan(plan_id, user, plan_days, user_service, plaid_client).await,
    )
}

#[delete("/plan/{id}")]
pub async fn delete_plan(
    Path(plan_id): Path<String>,
    user: User,
    user_service: Data<UserService>,
) -> HttpResponse {
    crate::common::into_response_res(PlansService::delete_plan(plan_id, user, user_service).await)
}

#[put("/plan/{id}")]
pub async fn update_plan(
    Path(plan_id): Path<String>,
    payload: Json<PlanNewPayload>,
    user: User,
    user_service: Data<UserService>,
    plaid_client: Data<Arc<Mutex<ApiClient>>>,
) -> HttpResponse {
    crate::common::into_response_res(
        PlansService::update_plan(
            payload.into_inner(),
            plan_id,
            user,
            365,
            user_service,
            plaid_client,
        )
        .await,
    )
}

#[put("/plan/{id}/{days}")]
pub async fn update_plan_with_days(
    Path((plan_id, plan_days)): Path<(String, i64)>,
    payload: Json<PlanNewPayload>,
    user: User,
    user_service: Data<UserService>,
    plaid_client: Data<Arc<Mutex<ApiClient>>>,
) -> HttpResponse {
    crate::common::into_response_res(
        PlansService::update_plan(
            payload.into_inner(),
            plan_id,
            user,
            plan_days,
            user_service,
            plaid_client,
        )
        .await,
    )
}

#[post("/plan/new")]
pub async fn create_new_plan(
    user: User,
    payload: Json<PlanNewPayload>,
    user_service: Data<UserService>,
    plaid_client: Data<Arc<Mutex<ApiClient>>>,
) -> HttpResponse {
    crate::common::into_response_res(
        PlansService::new_plan(payload.into_inner(), user, 365, user_service, plaid_client).await,
    )
}

#[post("/plan/new/{days}")]
pub async fn create_new_plan_with_days(
    Path(plan_days): Path<i64>,
    user: User,
    payload: Json<PlanNewPayload>,
    user_service: Data<UserService>,
    plaid_client: Data<Arc<Mutex<ApiClient>>>,
) -> HttpResponse {
    crate::common::into_response_res(
        PlansService::new_plan(
            payload.into_inner(),
            user,
            plan_days,
            user_service,
            plaid_client,
        )
        .await,
    )
}

pub fn init_routes(config: &mut ServiceConfig) {
    config.service(get_example);
    config.service(create_new_plan);
    config.service(get_plans);
    config.service(get_plan);
    config.service(update_plan);
    config.service(delete_plan);
    //config.service(create_new_plan_with_days);
    //config.service(update_plan_with_days);
    //config.service(get_plan_with_days);
}
