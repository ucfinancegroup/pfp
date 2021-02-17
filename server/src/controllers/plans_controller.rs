use crate::models::plan_model::*;
use crate::models::recurring_model::Recurring;
use crate::models::user_model::User;
use crate::services::{plans::PlansService, timeseries::TimeseriesService, users::UserService};
use actix_web::{
    delete, get, post, put,
    web::{Data, Path, ServiceConfig},
    HttpResponse,
};
use actix_web_validator::{Json, Validate};
use serde::{Deserialize, Serialize};

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
    crate::common::into_response(TimeseriesService::generate_sample_plan())
}

#[get("/plans")]
pub async fn get_plans(user: User) -> HttpResponse {
    crate::common::into_response(user.plans)
}

#[get("/plan/{id}")]
pub async fn get_plan(user: User, Path(plan_id): Path<String>) -> HttpResponse {
    crate::common::into_response_res(PlansService::get_plan(plan_id, user).await)
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
) -> HttpResponse {
    crate::common::into_response_res(
        PlansService::update_plan(plan_id, payload.into_inner(), user, user_service).await,
    )
}

#[post("/plan/new")]
pub async fn create_new_plan(
    user: User,
    payload: Json<PlanNewPayload>,
    user_service: Data<UserService>,
) -> HttpResponse {
    crate::common::into_response_res(
        PlansService::new_plan(payload.into_inner(), user, user_service).await,
    )
}

pub fn init_routes(config: &mut ServiceConfig) {
    config.service(get_example);
    config.service(create_new_plan);
    config.service(get_plans);
    config.service(get_plan);
    config.service(update_plan);
    config.service(delete_plan);
}
