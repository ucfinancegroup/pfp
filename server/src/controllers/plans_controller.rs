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

#[derive(Validate, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlanNewPayload {
    pub name: String,
    #[validate]
    pub recurrings: Vec<Recurring>,
    #[validate(length(min = 1))]
    pub allocations: Vec<Allocation>,
    #[validate]
    pub events: Vec<Event>,
}

#[derive(Validate, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlanUpdatePayload {
    pub name: Option<String>,
    #[validate]
    pub recurrings: Option<Vec<Recurring>>,
    #[validate(length(min = 1))]
    pub allocations: Option<Vec<Allocation>>,
    #[validate]
    pub events: Option<Vec<Event>>,
}

impl Into<Plan> for PlanNewPayload {
    fn into(self) -> Plan {
        let p = Plan {
            id: None,
            name: self.name,
            recurrings: self.recurrings,
            allocations: self.allocations,
            events: self.events,
        };

        p.ensure_ids()
    }
}

#[get("/event/examples")]
pub async fn get_example_events(_: User) -> HttpResponse {
    crate::common::into_response(PlansService::generate_sample_events())
}

#[get("/plan/example")]
pub async fn get_example_plan(_: User) -> HttpResponse {
    crate::common::into_response(PlansService::generate_sample_plan().ensure_ids())
}

#[get("/plan/plaid")]
pub async fn get_allocations_from_plaid(
    user: User,
    user_service: Data<UserService>,
    plaid_client: Data<ApiClient>,
) -> HttpResponse {
    crate::common::into_response_res(
        PlansService::update_plaid_allocation(user, 365, user_service, plaid_client).await,
    )
}

#[get("/plan/plaid/{days}")]
pub async fn get_allocations_from_plaid_with_days(
    Path(plan_days): Path<i64>,
    user: User,
    user_service: Data<UserService>,
    plaid_client: Data<ApiClient>,
) -> HttpResponse {
    crate::common::into_response_res(
        PlansService::update_plaid_allocation(user, plan_days, user_service, plaid_client).await,
    )
}

#[get("/plan")]
pub async fn get_plan(
    user: User,
    user_service: Data<UserService>,
    plaid_client: Data<ApiClient>,
) -> HttpResponse {
    crate::common::into_response_res(
        PlansService::get_plan(user, 365, user_service, plaid_client).await,
    )
}

#[get("/plan/{days}")]
pub async fn get_plan_with_days(
    user: User,
    Path(plan_days): Path<i64>,
    user_service: Data<UserService>,
    plaid_client: Data<ApiClient>,
) -> HttpResponse {
    crate::common::into_response_res(
        PlansService::get_plan(user, plan_days, user_service, plaid_client).await,
    )
}

#[delete("/plan")]
pub async fn delete_plan(user: User, user_service: Data<UserService>) -> HttpResponse {
    crate::common::into_response_res(PlansService::delete_plan(user, user_service).await)
}

#[post("/plan")]
pub async fn create_new_plan(
    user: User,
    payload: Json<PlanNewPayload>,
    user_service: Data<UserService>,
    plaid_client: Data<ApiClient>,
) -> HttpResponse {
    crate::common::into_response_res(
        PlansService::new_plan(payload.into_inner(), user, 365, user_service, plaid_client).await,
    )
}

#[post("/plan/{days}")]
pub async fn create_new_plan_with_days(
    Path(plan_days): Path<i64>,
    user: User,
    payload: Json<PlanNewPayload>,
    user_service: Data<UserService>,
    plaid_client: Data<ApiClient>,
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

#[put("/plan")]
pub async fn update_plan(
    user: User,
    payload: Json<PlanUpdatePayload>,
    user_service: Data<UserService>,
    plaid_client: Data<ApiClient>,
) -> HttpResponse {
    crate::common::into_response_res(
        PlansService::update_plan(payload.into_inner(), user, 365, user_service, plaid_client)
            .await,
    )
}

#[put("/plan/{days}")]
pub async fn update_plan_with_days(
    Path(plan_days): Path<i64>,
    user: User,
    payload: Json<PlanUpdatePayload>,
    user_service: Data<UserService>,
    plaid_client: Data<ApiClient>,
) -> HttpResponse {
    crate::common::into_response_res(
        PlansService::update_plan(
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
    config.service(get_example_plan);
    config.service(get_example_events);
    config.service(get_allocations_from_plaid);
    config.service(get_allocations_from_plaid_with_days);
    config.service(get_plan);
    config.service(get_plan_with_days);
    config.service(delete_plan);
    config.service(create_new_plan);
    config.service(create_new_plan_with_days);
    config.service(update_plan);
    config.service(update_plan_with_days);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_allocation_vector_validation() {
        let p = PlanUpdatePayload {
            name: None,
            recurrings: None,
            allocations: Some(vec![]),
            events: None,
        };

        assert!(p.validate().is_err());
    }
}
