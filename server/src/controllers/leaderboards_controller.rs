use crate::models::user_model::User;
use crate::services::db::DatabaseService;
use crate::services::leaderboards::LeaderboardService;
use actix_web::{
  get,
  web::{Data, Path},
  HttpResponse,
};

#[get("/leaderboard/{board}")]
pub async fn get_leaderboard(
  user: User,
  board: Path<String>,
  leaderboards: Data<LeaderboardService>,
) -> HttpResponse {
  crate::common::into_response(
    leaderboards.into_inner().get_ranking(board.to_string(), &user).await,
  )
}

use actix_web::web::ServiceConfig;
pub fn init_routes(config: &mut ServiceConfig) {
  config.service(get_leaderboard);
}
