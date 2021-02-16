use crate::models::plan_model::{Allocation, Event, Plan};
use crate::models::recurring_model::Recurring;
use actix_web::web::ServiceConfig;
use actix_web::web::{Data, Path};
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

#[post("/plans/new")]
pub async fn create_new_plan(user: User, payload: PlanNewPayload) -> HttpResponse {}

#[put("/plans/edit")]
pub async fn edit_plan(user: User, payload: PlanUpdatePayload) -> HttpResponse {}
*/
pub fn init_routes(config: &mut ServiceConfig) {
    //config.service(get_example);
}
