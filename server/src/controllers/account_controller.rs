use crate::models::{goal_model::*, user_model::User};
use crate::services::{goals::GoalService, users::UserService};
use actix_web::{
  delete, get, post, put,
  web::{Data, Path},
  HttpResponse,
};
use actix_web_validator::{Json, Validate};
use serde::{Deserialize, Serialize};
use validator::ValidationError;

#[get("/accounts")]
pub async fn get_accounts(
    mut user: User,
    user_service: Data<UserService>,
) -> HttpResponse {
    crate::common::into_response_res(user_service.get_accounts(&mut user).await)
}

// you add the services here.
use actix_web::web::ServiceConfig;
pub fn init_routes(config: &mut ServiceConfig) {
  config.service(get_snapshots);
}
