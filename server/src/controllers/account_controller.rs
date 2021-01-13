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
    user: User,
    user_service: Data<UserService>,
) -> HttpResponse {
    crate::common::into_response_res(user_service.get_accounts(user).await)
}

#[put("/accounts/{id}")]
pub async fn update_accounts(
    user: User,
    user_service: Data<UserService>,
    id: Path<String>,
    payload: Json<AccountNewPayload>,
) -> HttpResponse {
    crate::common::into_response_res(user_service.update_accounts(user, id.into_inner(), payload.into_inner()).await)
}

// you add the services here.
use actix_web::web::ServiceConfig;
pub fn init_routes(config: &mut ServiceConfig) {
  config.service(get_snapshots);
}
