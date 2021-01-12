pub mod goal_controller;
pub mod plaid_controller;
pub mod recurring_controller;
pub mod snapshot_controller;
pub mod user_controller;

use actix_web::web::ServiceConfig;
pub fn configure(config: &mut ServiceConfig) {
  user_controller::init_routes(config);
  plaid_controller::init_routes(config);
  snapshot_controller::init_routes(config);
  recurring_controller::init_routes(config);
  goal_controller::init_routes(config);
}
