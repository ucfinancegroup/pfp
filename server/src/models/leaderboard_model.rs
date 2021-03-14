use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Ranking {
  pub leaderboard_type: BoardTypes,
  pub percentile: f64,
  pub description: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum BoardTypes {
  Savings,
  Spending,
  Income,
}
