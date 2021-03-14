use crate::common::errors::ApiError;
use crate::models::leaderboard_model::Ranking;
use crate::models::user_model::User;
use crate::services::db;
use crate::services::db::DatabaseService;
use crate::services::insights::similar_user;

#[derive(Clone)]
pub struct LeaderboardService {
  db: DatabaseService,
}

#[allow(non_snake_case)]
impl LeaderboardService {
  pub async fn new(db: &db::DatabaseService) -> LeaderboardService {
    LeaderboardService { db: db.clone() }
  }

  pub async fn get_ranking(&self, board: String, user: &User) -> Result<Ranking, ApiError> {
    let board = board.to_lowercase();
    if board == "savings" || board == "spending" || board == "income" {
      similar_user::generate_ranking(user, &self.db, board)
        .await
        .map_err(|err| err.into())
    } else {
      Err(ApiError::new(
        400,
        "Leaderboard type must be savings, spending, or income.".to_string(),
      ))
    }
  }
}
