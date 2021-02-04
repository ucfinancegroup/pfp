use crate::common::Money;
use crate::models::user_model::User;
use crate::services::finchplaid::ApiClient;
use crate::services::{timeseries::TimeseriesService, users::UserService};
use actix_web::web::Data;
use actix_web::{get, HttpResponse};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

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
pub async fn get_timeseries(
    user: User,
    user_service: Data<UserService>,
    plaid_client: Data<Arc<Mutex<ApiClient>>>,
) -> HttpResponse {
    crate::common::into_response_res(
        TimeseriesService::get_timeseries(user, 365, user_service, plaid_client).await,
    )
}

// you add the services here.
use actix_web::web::ServiceConfig;
pub fn init_routes(config: &mut ServiceConfig) {
    config.service(get_example);
    config.service(get_timeseries);
}
