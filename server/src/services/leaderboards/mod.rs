#[allow(non_snake_case)]
pub mod LeaderboardService {
  use crate::common::errors::ApiError;
  use crate::models::user_model::User;

  pub async fn get_ranking(board: String, user: &User) -> Result<String, ApiError> {
    Ok(board)
  }
}
