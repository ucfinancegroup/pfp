use actix_web::web::ServiceConfig;
use actix_web::web::{Data, Path};

pub struct PlanNewPayload {}
pub struct PlanUpdatePayload {}

#[get("/plans/example")]
pub async fn get_example(_: User) -> HttpResponse {}

#[get("/plans")]
pub async fn get_plans(user: User) -> HttpResponse {}

#[post("/plans/new")]
pub async fn create_new_plan(user: User, payload: PlanNewPayload) -> HttpResponse {}

#[put("/plans/edit")]
pub async fn edit_plan(user: User, payload: PlanUpdatePayload) -> HttpResponse {}

pub fn init_routes(config: &mut ServiceConfig) {
    config.service(get_example);
}
