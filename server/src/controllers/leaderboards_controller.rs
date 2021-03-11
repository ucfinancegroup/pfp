use crate::models::{recurring_model::*, user_model::User};
use crate::services::leaderboards::LeaderboardService;
use actix_web::{
  get,
  web::{Path},
  HttpResponse,
};

#[get("/leaderboard/{board}")]
pub async fn get_leaderboard(user: User, board: Path<String>) -> HttpResponse {
    crate::common::into_response(LeaderboardService::get_ranking(board.to_string(), &user).await)
}

use actix_web::web::ServiceConfig;
pub fn init_routes(config: &mut ServiceConfig) {
  config.service(get_leaderboard);
}
