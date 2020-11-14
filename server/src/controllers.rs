pub mod plaid_controller;
pub mod user_controller;

use actix_web::web::ServiceConfig;
pub fn configure(config: &mut ServiceConfig) {
  user_controller::init_routes(config);
  plaid_controller::init_routes(config);
}
