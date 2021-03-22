use crate::models::leaderboard_model::BoardTypes;
use crate::models::user_model::User;
use crate::services::leaderboards::LeaderboardService;
use actix_web::{
  get,
  web::{Data, Path},
  HttpResponse,
};

#[get("/leaderboard/{board}")]
pub async fn get_leaderboard(
  user: User,
  board: Path<BoardTypes>,
  leaderboards: Data<LeaderboardService>,
) -> HttpResponse {
  crate::common::into_response_res(
    leaderboards
      .into_inner()
      .get_ranking(board.into_inner(), user)
      .await,
  )
}

use actix_web::web::ServiceConfig;
pub fn init_routes(config: &mut ServiceConfig) {
  config.service(get_leaderboard);
}
