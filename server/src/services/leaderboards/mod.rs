use crate::common::errors::ApiError;
use crate::models::leaderboard_model::{BoardType, Ranking};
use crate::models::user_model::User;
use crate::services::db::DatabaseService;
use crate::services::insights::common;

mod similar_user;

use crate::services::db;

use wither::mongodb::Database;

#[derive(Clone)]
pub struct LeaderboardService {
  db: Database,
}

#[allow(non_snake_case)]
impl LeaderboardService {
  pub async fn new(db: &db::DatabaseService) -> LeaderboardService {
    LeaderboardService { db: db.db.clone() }
  }

  pub async fn get_ranking(&self, board: String, user: &User) -> Result<Ranking, ApiError> {
    let board_type = board.to_lowercase();
    if board_type == "savings" || board_type == "spending" || board_type == "income" {
      similar_user::generate_ranking(user, &self.db, BoardType::Savings)
        .await
        .map_err(|err| err.into())
    } else {
      Err(ApiError::new(
        400,
        "Leaderboard type must be savings, checking, or income.".to_string(),
      ))
    }
  }
}
