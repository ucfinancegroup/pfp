use crate::common::Money;
use crate::models::user_model::User;
use crate::services::timeseries::TimeseriesService;
use actix_web::{get, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TimeseriesEntry {
    pub date: i64,
    pub net_worth: Money,
}

#[get("/timeseries/example")]
pub async fn get_example(_: User) -> HttpResponse {
    crate::common::into_response(TimeseriesService::get_example())
}

// you add the services here.
use actix_web::web::ServiceConfig;
pub fn init_routes(config: &mut ServiceConfig) {
    config.service(get_example);
}
