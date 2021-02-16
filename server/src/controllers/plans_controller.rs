use crate::models::plan_model::*;
use crate::models::recurring_model::Recurring;
use crate::models::user_model::User;
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

pub struct PlanUpdatePayload {}

/*
#[get("/plans/example")]
pub async fn get_example(_: User) -> HttpResponse {}

#[get("/plans")]
pub async fn get_plans(user: User) -> HttpResponse {}
*/

#[post("/plans/new")]
pub async fn create_new_plan(
    user: User,
    payload: Json<PlanNewPayload>,
    user_service: Data<UserService>,
) -> HttpResponse {
    crate::common::into_response_res(
        PlansService::new_plan(payload.into_inner(), user, user_service).await,
    )
}
/*
#[put("/plans/edit")]
pub async fn edit_plan(user: User, payload: PlanUpdatePayload) -> HttpResponse {}
*/
pub fn init_routes(config: &mut ServiceConfig) {
    //config.service(get_example);
}
