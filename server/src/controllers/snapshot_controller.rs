use crate::models::user_model::User;
use crate::services::{finchplaid::ApiClient, users::UserService};
use actix_web::{get, web::Data, HttpResponse};
use std::sync::{Arc, Mutex};

#[get("/snapshots")]
pub async fn get_snapshots(
  mut user: User,
  user_service: Data<UserService>,
  plaid_client: Data<Arc<Mutex<ApiClient>>>,
) -> HttpResponse {
  crate::common::into_response_res(user_service.get_snapshots(&mut user, plaid_client).await)
}

// you add the services here.
use actix_web::web::ServiceConfig;
pub fn init_routes(config: &mut ServiceConfig) {
  config.service(get_snapshots);
}
