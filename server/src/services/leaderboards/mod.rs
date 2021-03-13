#[allow(non_snake_case)]
pub mod LeaderboardService {
  use crate::common::errors::ApiError;
  use crate::models::leaderboard_model::Ranking;
  use crate::models::user_model::User;
  use crate::services::insights::common;

  pub async fn get_ranking(board: String, user: &User) -> Result<Ranking, ApiError> {
    let board_type = board.to_lowercase();
    if board_type == "savings" || board_type == "spending" || board_type == "income" {
      Ok(
        // TODO: Implement ranking method and call from here. Look at similar_user.rs for examples
        Ranking {
          leaderboard_type: board,
          percentile: 19.1,
          description: "Test".to_string(),
        },
      )
    } else {
      Err(ApiError::new(
        400,
        "Leaderboard type must be savings, checking, or income.".to_string(),
      ))
    }
  }
}
