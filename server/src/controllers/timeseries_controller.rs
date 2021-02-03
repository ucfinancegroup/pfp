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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TimeseriesResponse {
    pub start: i64,
    pub series: Vec<TimeseriesEntry>,
}

#[get("/timeseries/example")]
pub async fn get_example(_: User) -> HttpResponse {
    crate::common::into_response(TimeseriesService::get_example())
}

#[get("/timeseries/")]
pub async fn get_timeseries(user: User) -> HttpResponse {
    crate::common::into_response_res(TimeseriesService::get_timeseries(user, 365).await)
}

// you add the services here.
use actix_web::web::ServiceConfig;
pub fn init_routes(config: &mut ServiceConfig) {
    config.service(get_example);
    config.service(get_timeseries);
}
