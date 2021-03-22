use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Ranking {
  pub leaderboard_type: BoardTypes,
  pub percentile: f64,
  pub description: String,
  pub generation_time: i64,
}

impl Ranking {
  pub fn new(leaderboard_type: BoardTypes, percentile: f64, description: String) -> Ranking {
    Ranking {
      leaderboard_type: leaderboard_type,
      percentile,
      description,
      generation_time: Utc::now().timestamp(),
    }
  }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum BoardTypes {
  Savings,
  Spending,
  Income,
}
impl Default for BoardTypes {
  fn default() -> BoardTypes {
    BoardTypes::Savings
  }
}
